#![no_std]

mod macros;

pub mod console;
pub mod eeprom;
mod error;
pub mod io;
mod panic;
pub mod platform;

pub use error::{Error, Result};
pub use platform::Board;
pub use rot_bootloader_macros::entry;
