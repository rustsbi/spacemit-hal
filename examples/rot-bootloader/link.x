/* Vendor k1_defconfig: SPL_MAX_SIZE, SPL_BSS_* and SRAM stack top.
 * DDR v0.2: spacemit-firmware README's K1 patch, DDR_FIRMWARE_BASE. */
OUTPUT_ARCH(riscv)
ENTRY(_start)
_boot_hart_id = 0;

MEMORY {
    DDR_INFO (rw) : ORIGIN = 0xc0800000, LENGTH = 0x600
    IMAGE (rwx) : ORIGIN = 0xc0801000, LENGTH = 0x33000
    DDR_FW (rwx) : ORIGIN = 0xc082d000, LENGTH = 0xa000
    BSS (rw) : ORIGIN = 0xc0837000, LENGTH = 0x2000
    STACK (rw) : ORIGIN = 0xc0839000, LENGTH = 0x800
    DDR_STACK (rw) : ORIGIN = 0xc083a000, LENGTH = 0x6000
}

SECTIONS {
    .ddr_info (NOLOAD) : ALIGN(64) {
        __ddr_info = .;
        . += LENGTH(DDR_INFO);
    } > DDR_INFO
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
    __resident_end = .;
    .ddr_firmware : ALIGN(64) { *(.ddr_firmware) } > DDR_FW
    __image_end = SIZEOF(.ddr_firmware) == 0 ? __resident_end
        : ADDR(.ddr_firmware) + SIZEOF(.ddr_firmware);
    __ddr_fw_end = ORIGIN(DDR_FW) + LENGTH(DDR_FW);
    .bss (NOLOAD) : ALIGN(8) {
        __sbss = .;
        *(.bss .bss.* .sbss .sbss.* COMMON)
        . = ALIGN(8);
        __ebss = .;
    } > BSS
    .stack (NOLOAD) : ALIGN(16) {
        KEEP(*(.uninit.boot_stack))
    } > STACK
    .ddr_stack (NOLOAD) : ALIGN(16) {
        __ddr_stack_bottom = .;
        . += LENGTH(DDR_STACK);
        __ddr_stack_top = .;
    } > DDR_STACK
    /DISCARD/ : { *(.eh_frame .eh_frame_hdr) }
}
ASSERT(_start == ORIGIN(IMAGE), "incorrect K1 BootROM entry");
ASSERT(ORIGIN(DDR_INFO) + LENGTH(DDR_INFO) <= ORIGIN(IMAGE), "DDR metadata overlaps SPL");
ASSERT(__resident_end <= ORIGIN(IMAGE) + LENGTH(IMAGE), "SPL code overflow");
ASSERT(__image_end - ORIGIN(IMAGE) <= 0x34f00, "SPL payload exceeds vendor size limit");
ASSERT(__sidata == __sdata && __sdata % 8 == 0 && __edata % 8 == 0, "invalid data boundaries");
ASSERT(__sbss % 8 == 0 && __ebss % 8 == 0, "invalid BSS boundaries");
ASSERT(ADDR(.stack) % 16 == 0 && SIZEOF(.stack) == 2048, "invalid boot stack");
ASSERT(SIZEOF(.ddr_firmware) == 0 || __resident_end <= ORIGIN(DDR_FW), "SPL code overlaps DDR firmware");
ASSERT(ADDR(.ddr_firmware) == ORIGIN(DDR_FW), "incorrect DDR firmware entry");
ASSERT(SIZEOF(.ddr_firmware) == 0 || SIZEOF(.ddr_firmware) == 36248, "unexpected DDR v0.2 firmware size");
ASSERT(SIZEOF(.ddr_firmware) <= LENGTH(DDR_FW), "DDR firmware overflow");
ASSERT(__ddr_fw_end <= ORIGIN(BSS), "DDR workspace overlaps BSS");
ASSERT(ORIGIN(BSS) + LENGTH(BSS) <= ORIGIN(STACK), "BSS overlaps boot stack");
ASSERT(__ddr_stack_bottom >= ORIGIN(STACK) + LENGTH(STACK), "DDR stack overlaps boot stack");
ASSERT(__ddr_stack_top == 0xc0840000 && __ddr_stack_top % 16 == 0, "invalid DDR stack");
