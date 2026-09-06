//! RV32 data and BSS initialization.

// Called only by the boot hart; linker boundaries and the data load address
// are 4-byte aligned, with any end padding owned by the corresponding section.
// Data load/run ranges are identical or disjoint. No stack memory is accessed.
#[unsafe(naked)]
pub(super) unsafe extern "C" fn init_data_bss() {
    core::arch::naked_asm!(
        ".option push
        .option norelax
        lla     t0, __sidata
        lla     t1, __sdata
        lla     t2, __edata
        beq     t0, t1, 3f
    2:  bgeu    t1, t2, 3f
        lw      t3, 0(t0)
        sw      t3, 0(t1)
        addi    t0, t0, 4
        addi    t1, t1, 4
        j       2b
    3:  lla     t0, __sbss
        lla     t1, __ebss
    4:  bgeu    t0, t1, 5f
        sw      zero, 0(t0)
        addi    t0, t0, 4
        j       4b
    5:  ret
        .option pop",
    );
}
