//! V100 DesignWare APB I²C (`SET2000`, ACPI CID `INT33C2`).
//!
//! Register definitions and official manual references are provided by `dw-apb-i2c`.
//! The caller must establish clocks, reset, timing and optional capabilities.
//! K1/K3 TWSI clock and pad configuration does not apply.

pub use dw_apb_i2c::DwApbI2c as RegisterBlock;
