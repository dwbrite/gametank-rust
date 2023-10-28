MEMORY
{
	RAM (rwx) : ORIGIN = 0x0000, LENGTH = 2K
	FLASH (rw) : ORIGIN = 0x8000, LENGTH = 0x4000 /* only put flash before vector table */
}

SECTIONS
{
	.text : {
		*(.text*)
    } > FLASH = 0xFF

    .text : {
        *(.text*)
    } > FLASH

    .data : {
        *(.data*)
    } > RAM

    .bss : {
        *(.bss*)
    } > RAM

    .rodata : {
	    *(.rodata .rodata.*);
    } > FLASH
}