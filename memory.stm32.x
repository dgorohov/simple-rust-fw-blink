MEMORY
{
    FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 64K
    RAM (xrw)  : ORIGIN = 0x20000000, LENGTH = 20K
}

SECTIONS
{
    PROVIDE(_stack_start = ORIGIN(RAM) + LENGTH(RAM));
    .vector_table ORIGIN(FLASH) :
    {
        LONG(_stack_start);
        KEEP(*(.vector_table.reset_vector));
        __reset_vector = .;
    } > FLASH

    PROVIDE(_stext = ADDR(.vector_table) + SIZEOF(.vector_table));

    .text :
    {
        *(.text .text.*);
        . = ALIGN(4);
        __etext = .;
    } > FLASH

    .rodata __etext : ALIGN(4)
    {
        *(.rodata .rodata.*);
        . = ALIGN(4);
        __erodata = .;
    } > FLASH

    .data : AT(__erodata) ALIGN(4)
    {
        . = ALIGN(4);
        __sdata = .;
        *(.data .data.*);
        . = ALIGN(4);
        __edata = .;
    } > RAM

    __sidata = LOADADDR(.data);

    .bss : ALIGN(4)
    {
        . = ALIGN(4);
        __sbss = .;
        *(.bss .bss.*);
        . = ALIGN(4);
        __ebss = .;
    } > RAM

    . = ALIGN(4);
    __sheap = .;

    .got (NOLOAD) :
    {
        KEEP(*(.got .got.*));
    }

    /DISCARD/ :
    {
        *(.ARM.exidx);
        *(.ARM.exidx.*);
        *(.ARM.extab.*);
    }
}
