//! SpacemiT GPIO controllers.

mod flex_pad;
mod function;
mod inner;
mod input;
mod output;
mod pad_ext;
mod register;

pub use embedded_hal::digital::PinState;
pub use flex_pad::FlexPad;
pub use function::Function;
pub use input::Input;
pub use output::Output;
pub use pad_ext::PadExt;
pub use register::{RW1C, k1, k3};
