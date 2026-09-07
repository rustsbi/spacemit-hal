//! SpacemiT X100 startup for K3 application harts.

// Source: OpenSBI fw_base.S and spacemit_k3_cold_boot_allowed.
// https://github.com/spacemit-com/opensbi/blob/8bd2cbdf9856dbc1a990d36e26bf47411f356c42/firmware/fw_base.S#L335-L374
// https://github.com/spacemit-com/opensbi/blob/8bd2cbdf9856dbc1a990d36e26bf47411f356c42/platform/generic/spacemit/spacemit_k3.c
#[cfg(all(
    any(feature = "spacemit-x100", feature = "spacemit-a100"),
    target_arch = "riscv64",
    target_os = "none"
))]
pub use super::riscv::halt;

/// Initializes this hart and runs its boot or spawned entry.
///
/// # Safety
/// The caller must satisfy the [machine-entry contract](super) and execute on
/// a K3 X100 hart (hardware hart ID 0–7).
#[cfg(all(
    any(feature = "spacemit-x100", feature = "spacemit-a100"),
    target_arch = "riscv64",
    target_os = "none"
))]
#[cfg_attr(
    any(feature = "k3-bootrom", feature = "k3-cpu"),
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
        // ML2SETUP: local snoop port and L2 instruction/TLB prefetch.
        "andi    t0, tp, 3
        li      t1, 1
        sll     t1, t1, t0
        li      t0, 0x50000
        or      t1, t1, t0
        csrs    0x7f0, t1
        call    {initialize_coherency}",
        // MSETUP must be configured before the first Rust stack access or AMO.
        "li      t0, 0x10073
        csrs    0x7c0, t0
        fence   iorw, iorw",
        // Only X100 supports the vendor's writable misa.H enable sequence.
        "li      t0, 0x80
        csrs    misa, t0",
        // Application cores implement both floating-point and vector state.
        "li      t0, 0x6600
        csrs    mstatus, t0
        csrw    fcsr, zero
        csrw    vstart, zero
        csrw    vcsr, zero",
        // Return from Rust before parking this core.
        "call    {start_rust}
        tail    {halt}
        .option pop",
        start_rust = sym super::riscv::start_rust,
        initialize_coherency = sym super::riscv::initialize_coherency,
        halt = sym halt,
    );
}
