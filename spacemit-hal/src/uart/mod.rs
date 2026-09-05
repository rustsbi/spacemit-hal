//! 16550-compatible UARTs with XScale extensions on K1/M1 and K3.

pub mod blocking;
pub mod register;

pub use blocking::{BlockingUart, Error};
pub use register::RegisterBlock;
pub use uart16550;
