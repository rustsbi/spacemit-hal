//! Core-specific machine-mode startup.
//!
//! # Safety
//!
//! Align `__sidata`, `__sdata`, `__edata`, `__sbss`, and `__ebss` to XLEN / 8
//! bytes, padding section ends inside their allocations.
//!
//! Data load/run ranges must be identical or disjoint.
//!
//! Only `_boot_hart_id` may enter initially;
//! all other harts stay powered off until `Hart::spawn` publishes their state.
//!
//! Reserve the boot stack in writable RAM outside data/BSS, with its
//! 16-byte-aligned upper boundary named `__sstack`.
//!
//! Application cores require accessible CCI registers, shareable RAM and valid
//! reset cache state; retain coherent mappings thereafter.
//!
//! For `#[entry]`, select the matching SoC feature and satisfy its
//! `Peripherals::steal` contract; no peripheral tokens may exist before entry.

pub mod nuclei_n308;
pub mod spacemit_a100;
pub mod spacemit_rt24;
pub mod spacemit_x100;
pub mod spacemit_x60;

#[cfg(all(
    target_os = "none",
    any(target_arch = "riscv32", target_arch = "riscv64"),
))]
mod riscv;
