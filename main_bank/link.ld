MEMORY
{
	RAM (rwx) : ORIGIN = 0x0000, LENGTH = 2K
	FLASH (rw) : ORIGIN = 0xC000, LENGTH = 0x3FFA /* only put flash before vector table */
    VECTOR_TABLE (rw) : ORIGIN = 0xFFFA, LENGTH = 6
}

EXTERN(_INTERRUPTS);

SECTIONS
{
    .vector_table : {
        KEEP(*(.vector_table));
    } > VECTOR_TABLE

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