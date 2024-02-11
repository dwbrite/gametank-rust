.section .text
.global wait, null_interrupt

wait:
    .byte 0xCB
    RTS

null_interrupt:
    RTI