//! 16550-compatible UARTs with XScale extensions on K1/M1 and K3.

pub mod blocking;
pub mod config;
pub mod register;

pub use blocking::{BlockingUart, Error};
pub use config::{Baud, Config, Parity, StopBits, WordLength};
pub use register::RegisterBlock;
pub use uart16550;

/// A consumed UART token or borrowed register block.
pub trait Instance<'a> {
    /// Returns the UART registers for the consumed resource's lifetime.
    fn register_block(self) -> &'a RegisterBlock;
}

impl<'a> Instance<'a> for &'a RegisterBlock {
    fn register_block(self) -> &'a RegisterBlock {
        self
    }
}
