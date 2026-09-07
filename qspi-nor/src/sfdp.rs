//! Minimal JESD216 basic-parameter discovery.

// Protocol reference: spi-flash's SFDP discovery and JESD216 field definitions.
// https://github.com/adamgreig/spi-flash-rs
// https://github.com/torvalds/linux/blob/master/drivers/mtd/spi-nor/sfdp.h

use crate::{
    Error, Parameters,
    backend::{AddressSize, Backend, Transfer},
};

pub(crate) fn read<B: Backend>(
    backend: &mut B,
    offset: u32,
    bytes: &mut [u8],
) -> Result<(), Error<B::Error>> {
    if offset > 0x100_0000 || bytes.len() > (0x100_0000 - offset) as usize {
        return Err(Error::OutOfBounds);
    }
    crate::read_chunks(
        backend,
        Transfer::new(0x5a)
            .with_address(0, AddressSize::Three)
            .with_dummy_cycles(8),
        offset,
        bytes,
    )
}

pub(crate) fn discover<B: Backend>(backend: &mut B) -> Result<Parameters, Error<B::Error>> {
    let mut header = [0; 8];
    read(backend, 0, &mut header)?;
    if &header[..4] != b"SFDP" || header[5] != 1 || header[7] != 0xff {
        return Err(Error::InvalidSfdp);
    }
    let mut selected = None;
    for index in 0..=u32::from(header[6]) {
        let mut parameter = [0; 8];
        read(backend, 8 + index * 8, &mut parameter)?;
        // Select the newest compatible BFPT; unrelated parameter tables are skipped.
        if parameter[0] == 0 && parameter[7] == 0xff && parameter[2] == 1 && parameter[3] >= 9 {
            let pointer = u32::from_le_bytes([parameter[4], parameter[5], parameter[6], 0]);
            if selected.is_none_or(|(minor, _, _)| parameter[1] > minor) {
                selected = Some((parameter[1], pointer, parameter[3]));
            }
        }
    }
    let (_, pointer, length) = selected.ok_or(Error::InvalidSfdp)?;
    if pointer & 3 != 0 || u64::from(pointer) + u64::from(length) * 4 > 0x100_0000 {
        return Err(Error::InvalidSfdp);
    }
    let mut words = [0; 8];
    read(backend, pointer, &mut words)?;
    let first = u32::from_le_bytes(words[..4].try_into().unwrap());
    let density = u32::from_le_bytes(words[4..].try_into().unwrap());
    let bits = if density & (1 << 31) == 0 {
        u64::from(density) + 1
    } else {
        1u64.checked_shl(density & 0x7fff_ffff)
            .ok_or(Error::InvalidSfdp)?
    };
    if bits < 8 || bits % 8 != 0 {
        return Err(Error::InvalidSfdp);
    }
    // Do not change EN4B/bank registers or infer dedicated 4-byte opcodes.
    if bits / 8 > 0x100_0000 || (first >> 17) & 3 > 1 {
        return Err(Error::UnsupportedAddressing);
    }
    Ok(Parameters::new((bits / 8) as u32))
}
