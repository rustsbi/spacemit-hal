//! Core-specific machine-mode startup.
//!
//! # Safety
//! Align `__sidata`, `__sdata`, `__edata`, `__sbss`, and `__ebss` to XLEN / 8
//! bytes, padding section ends inside their allocations. Data load/run ranges
//! must be identical or disjoint; only the boot hart may write data/BSS until ready.
//! Each hart's stack must be writable and disjoint from data/BSS initialization.

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
