//! Peripheral abstractions for SpacemiT K1/M1 and K3 SoCs.

#![no_std]
#![deny(missing_docs)]

pub mod apbc;
pub mod apbs;
pub mod apmu;
pub mod ciu;
pub mod clock;
pub mod counter;
pub mod gpio;
pub mod i2c;
pub mod mfpr;
pub mod mpmu;
pub mod prelude;
pub mod qspi;
pub mod uart;
