/* Synthetic addresses for link/disassembly checks, not a board memory map. */
OUTPUT_ARCH(riscv)
ENTRY(_start)
/* Internal symbols use __; overridable configuration symbols use _. */
PROVIDE(_boot_hart_id = 0);
PROVIDE(_stext = 0x40000000);

SECTIONS
{
    . = _stext;
    .text : { KEEP(*(.text.entry)) *(.text .text.*) }
    .rodata : { *(.rodata .rodata.* .srodata .srodata.*) }

    .data (_stext + 0x20000) : AT(_stext + 0x10000)
    {
        __sdata = .;
        *(.data .data.* .sdata .sdata.*)
        . = ALIGN(8);
        __edata = .;
    }
    __sidata = LOADADDR(.data);
    __global_pointer$ = __sdata + 0x800;

    .bss (NOLOAD) : ALIGN(8)
    {
        __sbss = .;
        *(.bss .bss.* .sbss .sbss.*)
        . = ALIGN(8);
        __ebss = .;
    }
    .stack (NOLOAD) :
    {
        . = ALIGN(16);
        __estack = .;
        KEEP(*(.uninit.boot_stack))
        __sstack = .;
    }

    /DISCARD/ : { *(.eh_frame .eh_frame_hdr) }
}

ASSERT(ADDR(.rodata) + SIZEOF(.rodata) <= __sidata, "text overlaps data load image");
ASSERT(_start == ADDR(.text), "reset entry must be the first instruction");
ASSERT(__sidata + SIZEOF(.data) <= __sdata, "data load/run ranges overlap");
ASSERT(__edata <= __sbss && __ebss <= __estack, "RAM sections overlap");
/* Eight-byte boundaries satisfy both RV32 and RV64 startup. */
ASSERT(__sidata % 8 == 0 && __sdata % 8 == 0 && __edata % 8 == 0, "data must be XLEN-aligned");
ASSERT(__sbss % 8 == 0 && __ebss % 8 == 0, "BSS must be XLEN-aligned");
ASSERT(SIZEOF(.stack) == 2048 && __estack % 16 == 0, "invalid boot stack");
