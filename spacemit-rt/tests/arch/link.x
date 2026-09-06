/* Synthetic addresses for link/disassembly checks, not a board memory map. */
OUTPUT_ARCH(riscv)
ENTRY(_start)
/* Internal symbols use __; overridable configuration symbols use _. */
PROVIDE(_boot_hart_id = 0);
PROVIDE(_delay_hart_id = _boot_hart_id);
PROVIDE(_max_hart_id = 15);
PROVIDE(_hart_stack_size = 4096);
PROVIDE(_stext = 0x40000000);

SECTIONS
{
    . = _stext;
    .text : { KEEP(*(.text.entry)) *(.text .text.*) }
    .rodata : { *(.rodata .rodata.* .srodata .srodata.*) }

    /* Load this word at its VMA before releasing any harts, even the boot hart. */
    .boot_sync : ALIGN(4)
    {
        __sboot_sync = .;
        KEEP(*(.data.boot_sync))
        __eboot_sync = .;
    }

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
        . += (_max_hart_id + 1) * _hart_stack_size;
        __sstack = .;
        PROVIDE(_stack_start = __sstack);
    }

    /DISCARD/ : { *(.eh_frame .eh_frame_hdr) }
}

ASSERT(ADDR(.rodata) + SIZEOF(.rodata) <= __sidata, "text overlaps data load image");
ASSERT(LOADADDR(.boot_sync) == ADDR(.boot_sync), "boot sync must load at its VMA");
ASSERT(SIZEOF(.boot_sync) == 4 && __sboot_sync % 4 == 0, "boot sync must be one aligned word");
ASSERT(__eboot_sync <= __sidata, "boot sync overlaps data load image");
ASSERT(__sidata + SIZEOF(.data) <= __sdata, "data load/run ranges overlap");
ASSERT(__edata <= __sbss && __ebss <= __estack, "RAM sections overlap");
/* Eight-byte boundaries satisfy both RV32 and RV64 startup. */
ASSERT(__sidata % 8 == 0 && __sdata % 8 == 0 && __edata % 8 == 0, "data must be XLEN-aligned");
ASSERT(__sbss % 8 == 0 && __ebss % 8 == 0, "BSS must be XLEN-aligned");
ASSERT(_boot_hart_id >= 0 && _boot_hart_id <= _max_hart_id, "boot hart must have a stack");
ASSERT(_max_hart_id >= 0 && _max_hart_id < 0x7fffffff, "invalid maximum hart ID");
ASSERT(_hart_stack_size > 0 && _hart_stack_size < 0x80000000 && _hart_stack_size % 16 == 0, "invalid per-hart stack size");
ASSERT(_stack_start <= __sstack && _stack_start >= __estack + (_max_hart_id + 1) * _hart_stack_size, "insufficient per-hart stack storage");
ASSERT(_stack_start % 16 == 0, "stack must be 16-byte aligned");
