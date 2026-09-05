//! SpacemiT GPIO controllers.

mod function;
mod inner;
mod input;
mod output;
mod register;

pub use function::Function;
pub use input::Input;
pub use output::Output;
pub use register::{RW1C, k1, k3};
