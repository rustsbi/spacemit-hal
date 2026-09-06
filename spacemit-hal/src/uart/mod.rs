//! 16550-compatible UARTs with XScale extensions on K1/M1 and K3.

mod blocking;
mod config;
mod register;

pub use blocking::{BlockingUart, Error};
pub use config::{Baud, Config, Parity, StopBits, WordLength};
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
/// ClockId must name this UART's SoC and instance. Consuming register_block
/// must transfer exclusive access for 'a to its returned reference without
/// disabling hardware or invalidating its mapping; no owner may be recreated
/// through safe code while the returned register borrow remains live.
pub unsafe trait ClockedInstance<'a>: Instance<'a> {
    /// The matching SoC-specific UART clock identity.
    type ClockId: crate::clock::UartId;
}

impl<'a> Instance<'a> for &'a RegisterBlock {
    fn register_block(self) -> &'a RegisterBlock {
        self
    }
}
