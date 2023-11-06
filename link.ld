MEMORY
{
    ZP (rw) : ORIGIN = 0x0020, LENGTH = 0x00DF /* zero page up to 0x100, using 0x00 through 0x1F for imaginary reg */
	RAM (rwx) : ORIGIN = 0x0200, LENGTH = 0x1DFF /* RAM fills up to 0x2000 after the hardware stack */
	SCR (w) : ORIGIN = 0x2000, LENGTH = 0x0007 /* write only SCR, so the linker doesn't try to LDA $2000 */
	FLASH (rw) : ORIGIN = 0x8000, LENGTH = 0x7FF9 /* flash before vector table */
    VECTOR_TABLE (rw) : ORIGIN = 0xFFFA, LENGTH = 6
}

SECTIONS
{
    .vector_table : {
        KEEP(*(.vector_table));
    } > VECTOR_TABLE

	.text : {
		*(.text*)
    } > FLASH = 0xFF

    .rodata : {
        *(.rodata*);
    } > FLASH

    .init : {
        KEEP(*(.init))
    } > FLASH

    .data : {
        *(.data*)
    } > RAM

    .bss : {
        *(.bss*)
    } > RAM

/*
    .noinit (NOLOAD) : {
        *(.noinit*)
        _stack_end = .;
    } > RAM

    _stack_top = 0x1FFF;

    .zp : {
        *(.zp*)
    } > ZP
*/
}

# __rc0 and __rc1 are reserved as a 16 bit software stack pointer by llvm-mos
__rc0  = 0x00;
__rc1  = 0x01;

__rc2  = 0x02;
__rc3  = 0x03;
__rc4  = 0x04;
__rc5  = 0x05;
__rc6  = 0x06;
__rc7  = 0x07;
__rc8  = 0x08;
__rc9  = 0x09;
__rc10 = 0x0A;
__rc11 = 0x0B;
__rc12 = 0x0C;
__rc13 = 0x0D;
__rc14 = 0x0E;
__rc15 = 0x0F;
__rc16 = 0x10;
__rc17 = 0x11;
__rc18 = 0x12;
__rc19 = 0x13;
__rc20 = 0x14;
__rc21 = 0x15;
__rc22 = 0x16;
__rc23 = 0x17;
__rc24 = 0x18;
__rc25 = 0x19;
__rc26 = 0x1A;
__rc27 = 0x1B;
__rc28 = 0x1C;
__rc29 = 0x1D;
__rc30 = 0x1E;
__rc31 = 0x1F;
 

