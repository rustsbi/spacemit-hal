//! SpacemiT GPIO controllers.
//!
//! K1/M1 and K3 share the same broad GPIO capabilities, but not the same
//! register layout. Select the register block matching the target SoC.

mod register;

pub use register::{ReadWriteOneToClear, k1, k3};
