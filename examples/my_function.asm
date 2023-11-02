.section .text
.global my_function, null_interrupt

my_function:
    .byte $CB
    RTS

null_interrupt:
    RTI