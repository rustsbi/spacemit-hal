use qspi_nor::backend::{AddressSize, Backend, Transfer, Width};
use qspi_nor::{Config, Error, NorFlash, Parameters, ReadNorFlash};

struct Flash {
    sfdp: [u8; 128],
    trace: Vec<(Transfer, usize)>,
    busy: bool,
    error: Option<u8>,
}

impl Flash {
    fn new() -> Self {
        let mut sfdp = [0; 128];
        sfdp[..8].copy_from_slice(&[b'S', b'F', b'D', b'P', 6, 1, 0, 0xff]);
        sfdp[8..16].copy_from_slice(&[0, 6, 1, 9, 64, 0, 0, 0xff]);
        sfdp[68..72].copy_from_slice(&0x03ff_ffffu32.to_le_bytes());
        Self {
            sfdp,
            trace: Vec::new(),
            busy: false,
            error: None,
        }
    }
}

impl Backend for Flash {
    type Error = u8;
    const MAX_READ_SIZE: usize = 7;
    const MAX_WRITE_SIZE: usize = 7;

    fn command(&mut self, _: Transfer) -> Result<(), u8> {
        panic!("read-only NOR must not issue commands")
    }
    fn write(&mut self, _: Transfer, _: &[u8]) -> Result<(), u8> {
        panic!("read-only NOR must not write flash")
    }
    fn read(&mut self, transfer: Transfer, data: &mut [u8]) -> Result<(), u8> {
        assert!(data.len() <= Self::MAX_READ_SIZE);
        self.trace.push((transfer, data.len()));
        if let Some(error) = self.error {
            return Err(error);
        }
        match transfer.opcode {
            0x05 => data[0] = u8::from(self.busy),
            0x9f => data.copy_from_slice(&[0xc8, 0x40, 0x17]),
            0x5a => {
                assert_eq!(transfer.dummy_cycles, 8);
                let address = transfer.address.unwrap();
                assert_eq!(address.size, AddressSize::Three);
                let offset = address.value as usize;
                data.copy_from_slice(&self.sfdp[offset..offset + data.len()]);
            }
            0x03 | 0x13 | 0x6b => {
                let start = transfer.address.unwrap().value;
                for (i, value) in data.iter_mut().enumerate() {
                    *value = start.wrapping_add(i as u32) as u8;
                }
            }
            _ => panic!("unexpected opcode"),
        }
        Ok(())
    }
}

#[test]
fn discovers_capacity_and_chunks_only_addressable_reads() {
    let mut backend = Flash::new();
    {
        let mut flash = NorFlash::probe(&mut backend, Config::default()).unwrap();
        assert_eq!(flash.capacity(), 8 * 1024 * 1024);
        assert_eq!(flash.jedec_id().unwrap(), [0xc8, 0x40, 0x17]);
        let mut data = [0; 19];
        flash.read(5, &mut data).unwrap();
        assert_eq!(data, core::array::from_fn(|i| (5 + i) as u8));
        flash.read(flash.capacity() as u32, &mut []).unwrap();
        assert_eq!(flash.read(0x80_0000, &mut [0]), Err(Error::OutOfBounds));
        assert_eq!(flash.read(u32::MAX, &mut [0]), Err(Error::OutOfBounds));
    }
    let reads: Vec<_> = backend
        .trace
        .iter()
        .filter(|(t, _)| t.opcode == 3)
        .map(|(t, n)| (t.address.unwrap().value, *n))
        .collect();
    assert_eq!(reads, [(5, 7), (12, 7), (19, 5)]);
}

#[test]
fn rejects_malformed_and_unsupported_sfdp() {
    let mut backend = Flash::new();
    backend.sfdp[0] = 0;
    assert!(matches!(
        NorFlash::probe(backend, Config::default()),
        Err(Error::InvalidSfdp)
    ));
    let mut backend = Flash::new();
    backend.sfdp[11] = 1;
    assert!(matches!(
        NorFlash::probe(backend, Config::default()),
        Err(Error::InvalidSfdp)
    ));
    let mut backend = Flash::new();
    backend.sfdp[68..72].copy_from_slice(&0x8000_0040u32.to_le_bytes());
    assert!(matches!(
        NorFlash::probe(backend, Config::default()),
        Err(Error::InvalidSfdp)
    ));
    let mut backend = Flash::new();
    backend.sfdp[68..72].copy_from_slice(&0x8000_001cu32.to_le_bytes());
    assert!(matches!(
        NorFlash::probe(backend, Config::default()),
        Err(Error::UnsupportedAddressing)
    ));
}

#[test]
fn exponent_density_and_multiple_parameter_headers() {
    let mut backend = Flash::new();
    backend.sfdp[6] = 1;
    backend.sfdp[8] = 0x81;
    backend.sfdp[16..24].copy_from_slice(&[0, 6, 1, 9, 64, 0, 0, 0xff]);
    backend.sfdp[68..72].copy_from_slice(&0x8000_001au32.to_le_bytes());
    let flash = NorFlash::probe(backend, Config::default()).unwrap();
    assert_eq!(flash.capacity(), 8 * 1024 * 1024);
}

#[test]
fn reports_backend_failure_and_bounded_busy() {
    let mut backend = Flash::new();
    backend.error = Some(42);
    assert!(matches!(
        NorFlash::probe(&mut backend, Config::default()),
        Err(Error::Backend(42))
    ));
    backend.error = None;
    backend.busy = true;
    backend.trace.clear();
    assert!(matches!(
        NorFlash::probe(&mut backend, Config { poll_budget: 2 }),
        Err(Error::PollLimit)
    ));
    assert_eq!(backend.trace.len(), 2);
    backend.trace.clear();
    assert!(matches!(
        NorFlash::probe(&mut backend, Config { poll_budget: 0 }),
        Err(Error::InvalidConfig)
    ));
    assert!(backend.trace.is_empty());
}

#[test]
fn explicit_quad_and_four_byte_protocols_preserve_phases() {
    let mut backend = Flash::new();
    let mut read = Transfer::new(0x6b)
        .with_address(0, AddressSize::Three)
        .with_dummy_cycles(8);
    read.data_width = Width::Quad;
    let mut data = [0; 2];
    NorFlash::new(
        &mut backend,
        Parameters {
            capacity: 1024,
            read,
        },
        Config::default(),
    )
    .unwrap()
    .read(12, &mut data)
    .unwrap();
    assert_eq!(backend.trace.last().unwrap().0.data_width, Width::Quad);
    assert_eq!(backend.trace.last().unwrap().0.dummy_cycles, 8);
    let parameters = Parameters {
        capacity: 32 * 1024 * 1024,
        read: Transfer::new(0x13).with_address(0, AddressSize::Four),
    };
    NorFlash::new(&mut backend, parameters, Config::default())
        .unwrap()
        .read(0x100_0001, &mut data)
        .unwrap();
    assert_eq!(
        backend.trace.last().unwrap().0.address.unwrap().value,
        0x100_0001
    );
    assert!(matches!(
        NorFlash::new(
            backend,
            Parameters::new(32 * 1024 * 1024),
            Config::default()
        ),
        Err(Error::OutOfBounds)
    ));
}
