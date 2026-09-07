//! Polling Quad-SPI memory-controller access.
mod blocking;
mod lut;
mod register;
pub use blocking::{BlockingQspi, ChipSelect, Config, Error};
pub use qspi_nor::backend::{Address, AddressSize, Backend, Transfer, Width};
pub use register::{ModuleControl, RegisterBlock};

/// An exclusive QSPI register mapping.
///
/// # Safety
/// Transfer exclusive, aligned register access for 'a without recreating owners.
pub unsafe trait Instance<'a> {
    /// Consumes the controller or its mutable borrow.
    fn register_block(self) -> &'a RegisterBlock;
}
