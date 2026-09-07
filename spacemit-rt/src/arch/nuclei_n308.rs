//! Nuclei N308 startup for the K1/M1 real-time hart.

// Sources: ESOS n308/startup_gcc.S and n308/riscv-system-init.c;
// NMSIS riscv_encoding.h and core_feature_cache.h define the CSR fields.
// https://github.com/spacemit-com/esos/tree/eaf9afd83b27583b9bcafe0153028b32219518f5/libcpu/risc-v/spacemit/n308
// https://github.com/spacemit-com/esos/tree/eaf9afd83b27583b9bcafe0153028b32219518f5/components/nmsis/core/include
#[cfg(all(feature = "nuclei-n308", target_arch = "riscv32", target_os = "none"))]
pub use super::riscv::halt;

/// Initializes this hart and runs its boot or spawned entry.
///
/// # Safety
/// The caller must satisfy the [machine-entry contract](super) and execute on
/// the K1/M1 Nuclei N308 hart.
#[cfg(all(feature = "nuclei-n308", target_arch = "riscv32", target_os = "none"))]
#[cfg_attr(
    feature = "k1-mcu",
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
        // MCFG_INFO advertises optional I/D cache presence.
        "csrr    t0, 0xfc2
        andi    t1, t0, 0x200
        beqz    t1, 2f
        csrsi   0x7ca, 1
    2:  andi    t1, t0, 0x400
        beqz    t1, 3f
        li      t1, 0x10000
        csrs    0x7ca, t1
        3:",
        // MMISC_CTL.BPU, not the unrelated A100 PERF_CTRL at the same CSR number.
        "csrsi   0x7d0, 8
        csrci   mcountinhibit, 5
        fence   iorw, iorw",
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
