EXTERN(__main_void);
EXTERN(main);
PROVIDE(_start = __main_void);
PROVIDE(_start = main);
ENTRY(_start)
SECTIONS {
    . = 0x200000;
    . = ALIGN(0x1000);
    .text : {
        *(.text.entry)
        *(.text*)
    }
    . = ALIGN(0x1000);
    .rodata : { *(.rodata*) }
    . = ALIGN(0x1000);
    .data : { *(.data*) }
    .bss : {
        *(.bss*)
        *(COMMON)
    }

    /DISCARD/ : {
        *(.eh_frame)
        *(.eh_frame_hdr)
        *(.note*)
        *(.comment*)
    }
}
