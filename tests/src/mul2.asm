; Calculate 8x8 multiplication
; Example based on
; https://www.lysator.liu.se/~nisse/misc/6502-mul.html

        JMP start


!source "src/common/test_routines.asm"


factor1 !byte 8
factor2 !byte 8


start:  LDA #0
        LDX #$8
        LSR factor1

loop:   BCC no_add
        CLC
        ADC factor2

no_add: ROR
        ROR factor1
        DEX
        BNE loop
        STA factor2

test:   LDA factor1
        CMP #$40
        PHP
        JSR bneerr
        JSR next

ok:     LDY #0

end:    LDA test_count
        PLP
