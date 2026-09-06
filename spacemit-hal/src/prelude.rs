//! Common extension traits.

pub use crate::gpio::PadExt as _;
pub use crate::uart::UartExt as _;
pub use embedded_hal::digital::{InputPin as _, OutputPin as _};
pub use embedded_io::{Read as _, Write as _};
