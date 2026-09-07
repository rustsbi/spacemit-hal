/* K1 vendor configs/k1_defconfig: SRAM entry, BSS and stack reservations. */
OUTPUT_ARCH(riscv)
ENTRY(_start)
_boot_hart_id = 0;

MEMORY {
    IMAGE (rwx) : ORIGIN = 0xc0801000, LENGTH = 0x20000
    BSS (rw) : ORIGIN = 0xc0837000, LENGTH = 0x1000
    STACK (rw) : ORIGIN = 0xc0839000, LENGTH = 0x7000
}

SECTIONS {
    .text : ALIGN(8) { KEEP(*(.text.entry)) *(.text .text.*) } > IMAGE
    .rodata : ALIGN(8) { *(.rodata .rodata.* .srodata .srodata.*) } > IMAGE
    .data : ALIGN(8) {
        __sdata = .;
        *(.data .data.* .sdata .sdata.*)
        . = ALIGN(8);
        __edata = .;
    } > IMAGE
    __sidata = LOADADDR(.data);
    __global_pointer$ = __sdata + 0x800;
    __image_end = .;
    .bss (NOLOAD) : ALIGN(8) {
        __sbss = .;
        *(.bss .bss.* .sbss .sbss.* COMMON)
        . = ALIGN(8);
        __ebss = .;
    } > BSS
    .stack (NOLOAD) : ALIGN(16) {
        KEEP(*(.uninit.boot_stack))
    } > STACK
    /DISCARD/ : { *(.eh_frame .eh_frame_hdr) }
}
ASSERT(_start == ORIGIN(IMAGE), "incorrect K1 BootROM entry");
ASSERT(__image_end <= ORIGIN(IMAGE) + LENGTH(IMAGE), "SRAM image overflow");
ASSERT(__sidata == __sdata && __sdata % 8 == 0 && __edata % 8 == 0, "invalid data boundaries");
ASSERT(__sbss % 8 == 0 && __ebss % 8 == 0, "invalid BSS boundaries");
ASSERT(ADDR(.stack) % 16 == 0 && SIZEOF(.stack) == 2048, "invalid boot stack");
