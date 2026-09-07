//! Common extension traits.

pub use crate::gpio::PadExt as _;
pub use crate::i2c::{I2cExt as _, IntoI2c as _};
pub use crate::uart::UartExt as _;
pub use embedded_hal::delay::DelayNs as _;
pub use embedded_hal::digital::{InputPin as _, OutputPin as _};
pub use embedded_hal::i2c::I2c as _;
pub use embedded_io::{Read as _, Write as _};
