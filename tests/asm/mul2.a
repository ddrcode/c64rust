; Calculate 8x8 multiplication
; Example based on 
; https://www.lysator.liu.se/~nisse/misc/6502-mul.html

        JMP start

factor1 !byte 8
factor2 !byte 8


start   LDA #0
        LDX #$8
        LSR factor1
loop    BCC no_add
        CLC
        ADC factor2
no_add  ROR
        ROR factor1
        DEX
        BNE loop
        STA factor2
end     LDA factor1

