//! Controller-independent, single-chip QSPI transactions.

/// Number of signal lines used by one SDR phase.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Width {
    /// One signal line.
    #[default]
    Single,
    /// Two signal lines.
    Dual,
    /// Four signal lines.
    Quad,
}

/// Serial address length in bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AddressSize {
    /// Eight address bits.
    One = 1,
    /// Sixteen address bits.
    Two = 2,
    /// Twenty-four address bits.
    Three = 3,
    /// Thirty-two address bits.
    Four = 4,
}

/// An MSB-first serial address phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Address {
    /// Address value without truncation.
    pub value: u32,
    /// Number of transmitted address bytes.
    pub size: AddressSize,
    /// Address-phase signal width.
    pub width: Width,
}

/// An SDR command, optional address, dummy clocks, and optional data phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Transfer {
    /// Eight-bit instruction opcode.
    pub opcode: u8,
    /// Instruction-phase signal width.
    pub instruction_width: Width,
    /// Optional address phase.
    pub address: Option<Address>,
    /// Dummy SCK cycles, not bytes.
    pub dummy_cycles: u8,
    /// Dummy-phase signal width.
    pub dummy_width: Width,
    /// Data-phase signal width.
    pub data_width: Width,
}

impl Transfer {
    /// Creates a single-line command without an address or dummy clocks.
    pub const fn new(opcode: u8) -> Self {
        Self {
            opcode,
            instruction_width: Width::Single,
            address: None,
            dummy_cycles: 0,
            dummy_width: Width::Single,
            data_width: Width::Single,
        }
    }

    /// Adds a single-line address phase.
    pub const fn with_address(mut self, value: u32, size: AddressSize) -> Self {
        self.address = Some(Address {
            value,
            size,
            width: Width::Single,
        });
        self
    }

    /// Sets the number of dummy clock cycles.
    pub const fn with_dummy_cycles(mut self, cycles: u8) -> Self {
        self.dummy_cycles = cycles;
        self
    }
}

/// Executes single-CS transactions, returning idle or rejecting further use after failure.
pub trait Backend {
    /// Controller-specific failure.
    type Error: core::fmt::Debug;
    /// Maximum read payload accepting arbitrary byte alignment, greater than zero.
    const MAX_READ_SIZE: usize;
    /// Maximum write payload accepting arbitrary byte alignment.
    const MAX_WRITE_SIZE: usize;

    /// Executes one CS cycle without data.
    fn command(&mut self, transfer: Transfer) -> Result<(), Self::Error>;
    /// Reads exactly the buffer length within one CS cycle, without silently splitting it.
    fn read(&mut self, transfer: Transfer, data: &mut [u8]) -> Result<(), Self::Error>;
    /// Writes exactly the buffer length within one CS cycle, without silently splitting it.
    fn write(&mut self, transfer: Transfer, data: &[u8]) -> Result<(), Self::Error>;
}

impl<B: Backend + ?Sized> Backend for &mut B {
    type Error = B::Error;
    const MAX_READ_SIZE: usize = B::MAX_READ_SIZE;
    const MAX_WRITE_SIZE: usize = B::MAX_WRITE_SIZE;

    fn command(&mut self, transfer: Transfer) -> Result<(), Self::Error> {
        B::command(self, transfer)
    }
    fn read(&mut self, transfer: Transfer, data: &mut [u8]) -> Result<(), Self::Error> {
        B::read(self, transfer, data)
    }
    fn write(&mut self, transfer: Transfer, data: &[u8]) -> Result<(), Self::Error> {
        B::write(self, transfer, data)
    }
}
