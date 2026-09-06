//! SpacemiT A100 startup for K3 AI harts.

// Source: OpenSBI fw_base.S and the hart >= 8 tuning branch in
// spacemit_k3_cold_boot_allowed; A100 does not support misa.H.
// https://github.com/spacemit-com/opensbi/blob/8bd2cbdf9856dbc1a990d36e26bf47411f356c42/platform/generic/spacemit/spacemit_k3.c
#[cfg(all(feature = "spacemit-a100", target_arch = "riscv64", target_os = "none"))]
pub use super::riscv::halt;

/// Initializes the current hart and enters `__spacemit_rt_main()`.
///
/// # Safety
/// The caller must satisfy the [machine-entry contract](super) and execute on
/// a K3 A100 hart (hardware hart ID 8–15).
#[cfg(all(feature = "spacemit-a100", target_arch = "riscv64", target_os = "none"))]
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
        call    {initialize_stack}",
        // ML2SETUP: local snoop port and L2 instruction/TLB prefetch.
        "andi    t0, tp, 3
        li      t1, 1
        sll     t1, t1, t0
        li      t0, 0x50000
        or      t1, t1, t0
        csrs    0x7f0, t1",
        // PERF_CTRL: vector L1 bypass; PREFETCH_CTRL: L2 prefetch distance.
        "li      t0, 0x100000000
        csrs    0x7d0, t0
        li      t0, 0xc00
        csrs    0x7d1, t0",
        // ML2HINT: enable dependency handling and disable request merging.
        "csrci   0x7f7, 8
        csrsi   0x7f7, 4
        li      t0, 0x10073
        csrs    0x7c0, t0
        fence   iorw, iorw",
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
        halt = sym halt,
        initialize_stack = sym super::riscv::initialize_stack,
    );
}
