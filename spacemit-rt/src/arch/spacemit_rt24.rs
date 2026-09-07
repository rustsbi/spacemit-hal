//! SpacemiT RT24 startup for K3 real-time harts.

// ESOS uses standard machine CSRs for RT24 startup; its cache implementation
// provides no verified enable sequence, so retain the loader's cache state.
// In particular, 0x7c0 must not be treated as the X60/X100/A100 MSETUP register.
// https://github.com/spacemit-com/esos/blob/eaf9afd83b27583b9bcafe0153028b32219518f5/libcpu/risc-v/spacemit/rt24/startup_gcc.S
// https://github.com/spacemit-com/esos/blob/eaf9afd83b27583b9bcafe0153028b32219518f5/libcpu/risc-v/spacemit/rt24/riscv-cache.c
#[cfg(all(feature = "spacemit-rt24", target_arch = "riscv64", target_os = "none"))]
pub use super::riscv::halt;

/// Initializes this hart and runs its boot or spawned entry.
///
/// # Safety
/// The caller must satisfy the [machine-entry contract](super) and execute on
/// a K3 RT24 hart.
#[cfg(all(feature = "spacemit-rt24", target_arch = "riscv64", target_os = "none"))]
#[cfg_attr(
    feature = "k3-mcu",
    unsafe(export_name = "_start"),
    unsafe(link_section = ".text.entry")
)]
#[unsafe(naked)]
pub unsafe extern "C" fn start() -> ! {
    core::arch::naked_asm!(
        ".option push
        .option norelax
        csrci   mstatus, 8
        csrw    mie, zero",
        // Do not inherit MPRV from the loader.
        "li      t0, 0x20000
        csrc    mstatus, t0
        csrr    tp, mhartid",
        // This real-time core has floating-point state, but no vector extension.
        "li      t0, 0x6000
        csrs    mstatus, t0
        csrw    fcsr, zero",
        // Return from Rust before parking this core.
        "call    {start_rust}
        tail    {halt}
        .option pop",
        start_rust = sym super::riscv::start_rust,
        halt = sym halt,
    );
}
