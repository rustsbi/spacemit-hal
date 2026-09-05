//! Bare-metal runtime support for SpacemiT K1 and K3 SoCs.

#![no_std]
#![deny(missing_docs)]

#[macro_use]
mod macros;

/// SoC-specific peripheral ownership and address maps.
pub mod soc {
    pub mod k1;
    pub mod k3;
}
