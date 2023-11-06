.section .text
.global wait, null_interrupt

wait:
    .byte $CB
    RTS

null_interrupt:
    RTI