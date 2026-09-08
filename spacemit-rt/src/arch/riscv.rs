//! Shared RISC-V startup before entering Rust.

cfg_if::cfg_if! {
    if #[cfg(target_arch = "riscv64")] {
        mod rv64;
        use rv64::init_data_bss;
    } else if #[cfg(target_arch = "riscv32")] {
        mod rv32;
        use rv32::init_data_bss;
    }
}

unsafe extern "C" {
    static __sstack: u8;
    fn __spacemit_rt_main();
}

// OpenSBI bus-cci.c / bus-cci-550.c: cluster N uses CCI slave interface N.
// Configure this cluster before touching coherent RAM; spawn serializes releases.
#[cfg(all(
    target_arch = "riscv64",
    any(
        feature = "spacemit-x60",
        feature = "spacemit-x100",
        feature = "spacemit-a100"
    )
))]
#[unsafe(naked)]
pub(super) unsafe extern "C" fn initialize_coherency() {
    core::arch::naked_asm!(
        "li      t0, 0xd8500000
        srli    t1, tp, 2
        addi    t1, t1, 1
        slli    t1, t1, 12
        add     t1, t0, t1
        li      t2, 3
        sw      t2, 0(t1)
        fence   iorw, iorw
    2:  lw      t1, 12(t0)
        andi    t1, t1, 1
        bnez    t1, 2b
        fence   iorw, iorw
        ret",
    );
}

// Core setup has finished, tp holds mhartid, and no stack is required on entry.
// Only the boot hart initializes memory; secondaries consume published stacks.
#[unsafe(naked)]
pub(super) unsafe extern "C" fn start_rust() {
    core::arch::naked_asm!(
        ".option push
        .option norelax
        lla     gp, __global_pointer$
        lui     t0, %hi(_boot_hart_id)
        addi    t0, t0, %lo(_boot_hart_id)
        bne     tp, t0, 3f
        lla     sp, {boot_stack}
        addi    sp, sp, -16
        .if {register_bytes} == 8
        sd      ra, 0(sp)
        .else
        sw      ra, 0(sp)
        .endif
        call    {init_data_bss}
        fence.i
        call    {main}
        j       5f
    3:  li      t0, {hart_count}
        bgeu    tp, t0, 6f
        lla     t0, {mailboxes}
        li      t1, {mailbox_size}
        mul     t1, tp, t1
        add     t0, t0, t1",
        // Acquire the publication once, before the first access to this stack.
        ".if {register_bytes} == 8
        amoswap.d.aqrl sp, zero, (t0)
        .else
        amoswap.w.aqrl sp, zero, (t0)
        .endif
        beqz    sp, 6f
        addi    sp, sp, -16
        .if {register_bytes} == 8
        sd      ra, 0(sp)
        ld      t1, {entry_offset}(t0)
        ld      a0, {argument_offset}(t0)
        .else
        sw      ra, 0(sp)
        lw      t1, {entry_offset}(t0)
        lw      a0, {argument_offset}(t0)
        .endif
        fence.i
        jalr    ra, t1, 0
    5:  .if {register_bytes} == 8
        ld      ra, 0(sp)
        .else
        lw      ra, 0(sp)
        .endif
        addi    sp, sp, 16
        ret
    6:  tail    {halt}
        .option pop",
        register_bytes = const core::mem::size_of::<usize>(),
        boot_stack = sym __sstack,
        hart_count = const crate::hart::MAILBOXES.len(),
        mailboxes = sym crate::hart::MAILBOXES,
        mailbox_size = const core::mem::size_of::<crate::hart::Mailbox>(),
        entry_offset = const core::mem::offset_of!(crate::hart::Mailbox, entry),
        argument_offset = const core::mem::offset_of!(crate::hart::Mailbox, argument),
        init_data_bss = sym init_data_bss,
        main = sym __spacemit_rt_main,
        halt = sym halt,
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
