//! APBC register values and chip-specific layouts.

mod commons;
pub mod k1;
pub mod k3;

pub use commons::{TwsiClockReset, TwsiClockSource, UartClockReset, UartClockSource};
