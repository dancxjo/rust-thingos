EXTERN(__main_void);
EXTERN(main);
PROVIDE(_start = __main_void);
PROVIDE(_start = main);
ENTRY(_start)

SECTIONS {
    . = 0x200000;

    . = ALIGN(0x1000);
    .text : {
        *(.text .text.*)
    }

    . = ALIGN(0x1000);
    .rodata : {
        *(.rodata .rodata.*)
    }

    . = ALIGN(0x1000);
    .data : {
        *(.data .data.*)
    }

    .bss : {
        *(.bss .bss.*)
    }

    /DISCARD/ : {
        *(.eh_frame)
        *(.comment)
    }
}
