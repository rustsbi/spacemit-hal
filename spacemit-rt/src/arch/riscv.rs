//! Shared RISC-V startup before entering Rust.

use core::sync::atomic::AtomicU32;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "riscv64")] {
        mod rv64;
        use rv64::init_data_bss;
    } else if #[cfg(target_arch = "riscv32")] {
        mod rv32;
        use rv32::init_data_bss;
    }
}

// The loader initializes this separate, writable, coherent section before any
// hart enters; it must never be copied or cleared by the runtime itself.
#[used]
#[unsafe(link_section = ".data.boot_sync")]
static BOOT_READY: AtomicU32 = AtomicU32::new(0);

unsafe extern "C" {
    fn __spacemit_rt_main();
}

// Called from naked assembly with interrupts masked and valid inherited vectors.
// This helper returns through ra without reading or writing stack memory.
#[unsafe(naked)]
pub(super) unsafe extern "C" fn initialize_stack() {
    core::arch::naked_asm!(
        ".option push
        .option norelax
        csrr    tp, mhartid
        lui     t0, %hi(_max_hart_id)
        addi    t0, t0, %lo(_max_hart_id)
        bgtu    tp, t0, 2f
        lui     t0, %hi(_hart_stack_size)
        addi    t0, t0, %lo(_hart_stack_size)
        mul     t0, tp, t0
        lla     sp, _stack_start
        sub     sp, sp, t0
        andi    sp, sp, -16
        lla     gp, __global_pointer$
        ret",
        // An out-of-range hart has no allocated stack and cannot safely proceed.
        "2:  tail    {halt}
        .option pop",
        halt = sym halt,
    );
}

// Every caller has its own stack, gp, and tp, and completed its core setup.
// Only the configured boot hart initializes shared memory; all harts enter Rust.
// Keep ra in a 16-byte-aligned frame across both calls; stacks must be outside
// the data/BSS initialization ranges. Restore it before returning to core startup.
#[unsafe(naked)]
pub(super) unsafe extern "C" fn start_rust() {
    core::arch::naked_asm!(
        ".option push
        .option norelax
        addi    sp, sp, -16
        .if {register_bytes} == 8
        sd      ra, 0(sp)
        .else
        sw      ra, 0(sp)
        .endif
        lui     t0, %hi(_boot_hart_id)
        addi    t0, t0, %lo(_boot_hart_id)
        bne     tp, t0, 8f
        call    {init_data_bss}",
        // Publish completed data/BSS initialization with release ordering.
        "lla     t0, {boot_ready}
        li      t1, 1
        fence   rw, w
        sw      t1, 0(t0)
    8:  lla     t0, {boot_ready}
    9:  lw      t1, 0(t0)
        beqz    t1, 9b
        fence   r, rw
        fence.i
        call    {}
        .if {register_bytes} == 8
        ld      ra, 0(sp)
        .else
        lw      ra, 0(sp)
        .endif
        addi    sp, sp, 16
        ret
        .option pop",
        sym __spacemit_rt_main,
        register_bytes = const core::mem::size_of::<usize>(),
        boot_ready = sym BOOT_READY,
        init_data_bss = sym init_data_bss,
    );
}

/// Masks machine interrupts and parks the current hart.
///
/// # Safety
/// The caller must execute in M-mode and permit this hart to stop indefinitely.
#[unsafe(naked)]
pub unsafe extern "C" fn halt() -> ! {
    core::arch::naked_asm!(
        "csrci   mstatus, 8
        csrw    mie, zero
    2:  wfi
        j       2b",
    );
}
