ENTRY(_start)

SECTIONS
{
    . = 0x80000;

    .text : ALIGN(16)
    {
        KEEP(*(.text._start))
        *(.text .text.*)
    }

    .rodata : ALIGN(16)
    {
        *(.rodata .rodata.*)
    }

    .data : ALIGN(16)
    {
        *(.data .data.*)
    }

    .eh_frame : ALIGN(8)
    {
        *(.eh_frame*)
    }

    .bss (NOLOAD) : ALIGN(16)
    {
        *(.bss .bss.*)
        *(COMMON)

        . = ALIGN(16);

        __stack_bottom = .;
        . += 0x10000;
        __stack_top = .;
    }
}