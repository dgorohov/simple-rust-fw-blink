MEMORY
{
    FLASH : ORIGIN = 0x00000000, LENGTH = 1M
    RAM : ORIGIN = 0x20000000, LENGTH = 250K
}

ENTRY(Reset);
EXTERN(RESET_VECTOR);
EXTERN(__EXCEPTIONS);
EXTERN(__INTERRUPTS);
EXTERN(DefaultHandler);

#
# Provided by cortex-m-rt
#
EXTERN(HardFaultTrampoline);
PROVIDE(NonMaskableInt = DefaultHandler);
PROVIDE(MemoryManagement = DefaultHandler);
PROVIDE(BusFault = DefaultHandler);
PROVIDE(UsageFault = DefaultHandler);
PROVIDE(SecureFault = DefaultHandler);
PROVIDE(SVCall = DefaultHandler);
PROVIDE(DebugMonitor = DefaultHandler);
PROVIDE(PendSV = DefaultHandler);
PROVIDE(SysTick = DefaultHandler);
PROVIDE(DefaultHandler = DefaultHandler_);
PROVIDE(HardFault = HardFault_);
# PROVIDE(__pre_init = DefaultPreInit);

#
# Provided by nrf52840-pac
#
PROVIDE(CRYPTOCELL = DefaultHandler);
PROVIDE(POWER_CLOCK = DefaultHandler);
PROVIDE(RADIO = DefaultHandler);
PROVIDE(UARTE0_UART0 = DefaultHandler);
PROVIDE(UARTE1 = DefaultHandler);
PROVIDE(COMP_LPCOMP = DefaultHandler);
PROVIDE(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 = DefaultHandler);
PROVIDE(SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1 = DefaultHandler);
PROVIDE(SPIM2_SPIS2_SPI2 = DefaultHandler);
PROVIDE(SPIM3 = DefaultHandler);
PROVIDE(NFCT = DefaultHandler);
PROVIDE(GPIOTE = DefaultHandler);
PROVIDE(SAADC = DefaultHandler);
PROVIDE(TIMER0 = DefaultHandler);
PROVIDE(TIMER1 = DefaultHandler);
PROVIDE(TIMER2 = DefaultHandler);
PROVIDE(TIMER3 = DefaultHandler);
PROVIDE(TIMER4 = DefaultHandler);
PROVIDE(RTC0 = DefaultHandler);
PROVIDE(RTC1 = DefaultHandler);
PROVIDE(RTC2 = DefaultHandler);
PROVIDE(PWM0 = DefaultHandler);
PROVIDE(PWM1 = DefaultHandler);
PROVIDE(PWM2 = DefaultHandler);
PROVIDE(PWM3 = DefaultHandler);
PROVIDE(SWI0_EGU0 = DefaultHandler);
PROVIDE(SWI1_EGU1 = DefaultHandler);
PROVIDE(SWI2_EGU2 = DefaultHandler);
PROVIDE(SWI3_EGU3 = DefaultHandler);
PROVIDE(SWI4_EGU4 = DefaultHandler);
PROVIDE(SWI5_EGU5 = DefaultHandler);
PROVIDE(PDM = DefaultHandler);
PROVIDE(TEMP = DefaultHandler);
PROVIDE(RNG = DefaultHandler);
PROVIDE(ECB = DefaultHandler);
PROVIDE(CCM_AAR = DefaultHandler);
PROVIDE(FPU = DefaultHandler);
PROVIDE(USBD = DefaultHandler);
PROVIDE(QDEC = DefaultHandler);
PROVIDE(QSPI = DefaultHandler);
PROVIDE(WDT = DefaultHandler);
PROVIDE(MWU = DefaultHandler);
PROVIDE(I2S = DefaultHandler);

SECTIONS
{
    PROVIDE(_stack_start = ORIGIN(RAM) + LENGTH(RAM));
    .vector_table ORIGIN(FLASH) :
    {
        LONG(_stack_start);
        KEEP(*(.vector_table.reset_vector));
        __reset_vector = .;

        KEEP(*(.vector_table.exceptions));
        __eexceptions = .;

        KEEP(*(.vector_table.interrupts));
    } > FLASH

    PROVIDE(_stext = ADDR(.vector_table) + SIZEOF(.vector_table));

    .text :
    {
        *(.text .text.*);
        *(.HardFaultTrampoline);
        *(.HardFault.*);
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

ASSERT(SIZEOF(.got) == 0, "
ERROR(rt): .got section detected in the input object files
Dynamic relocations are not supported. If you are linking to C code compiled using
the 'cc' crate then modify your build script to compile the C code _without_
the -fPIC flag. See the documentation of the `cc::Build.pic` method for details.");