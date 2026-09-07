#![no_std]

mod macros;

pub mod console;
#[cfg(feature = "ddr")]
pub mod ddr;
pub mod eeprom;
mod error;
#[cfg(feature = "ddr")]
pub mod handoff;
#[cfg(feature = "ddr")]
pub mod image;
pub mod io;
mod panic;
pub mod platform;

pub use error::{Error, Result};
pub use platform::Board;
pub use rot_bootloader_macros::entry;

#[doc(hidden)]
pub use ufmt as __ufmt;
