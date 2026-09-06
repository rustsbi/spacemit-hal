//! SpacemiT X60 startup for K1/M1 application harts.

// Sources: OpenSBI K1 core_common.h, spacemit_cold_boot_allowed and
// sbi_hart_switch_mode (FEATURECTL); U-Boot x60/cache.c.
// https://github.com/spacemit-com/opensbi/tree/fc02b891b17b8bdc1273a39f80aa374cd99ba9a2
// https://github.com/spacemit-com/uboot-2022.10/blob/1fa1ca64e9705a3650bcc7c21f6666949290830f/arch/riscv/cpu/x60/cache.c
#[cfg(all(feature = "spacemit-x60", target_arch = "riscv64", target_os = "none"))]
pub use super::riscv::halt;

/// Initializes the current hart and enters `__spacemit_rt_main()`.
///
/// # Safety
/// The caller must satisfy the [machine-entry contract](super) and execute on
/// a K1/M1 X60 hart.
#[cfg(all(feature = "spacemit-x60", target_arch = "riscv64", target_os = "none"))]
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
        // ML2SETUP: enable this hart's snoop port before enabling its data cache.
        "andi    t0, tp, 3
        li      t1, 1
        sll     t1, t1, t0
        csrs    0x7f0, t1",
        // FEATURECTL: vendor K1 fence/cache fixes and vector LS dual-issue workaround.
        "li      t0, 0x800280
        csrs    0xbf9, t0",
        // MSETUP: D/I cache, branch prediction, prefetch, misaligned access and ECC.
        "li      t0, 0x10073
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
