//! 16550-compatible UARTs with XScale extensions on K1/M1 and K3.

mod blocking;
mod config;
mod ext;
mod pads;
mod register;

pub use blocking::{BlockingUart, Error};
pub use config::{Baud, Config, Parity, StopBits, WordLength};
pub use ext::UartExt;
pub use pads::{IntoReceive, IntoTransmit, Pads};
pub use register::RegisterBlock;
pub use uart16550;

/// A consumed UART token or borrowed register block.
pub trait Instance<'a> {
    /// Returns the UART registers for the consumed resource's lifetime.
    fn register_block(self) -> &'a RegisterBlock;
}

/// A UART instance whose identity matches an exclusive APBC clock token.
///
/// # Safety
/// ClockId must identify this UART; register_block must transfer exclusive access
/// for 'a without recreating owners, with valid mappings and no conflicting users
/// or DMA; power and upstream clocks must permit access whenever its APBC gate is enabled.
pub unsafe trait ClockedInstance<'a>: Instance<'a> {
    /// The matching SoC-specific UART clock identity.
    type ClockId: crate::clock::UartId;
}

impl<'a> Instance<'a> for &'a RegisterBlock {
    fn register_block(self) -> &'a RegisterBlock {
        self
    }
}
