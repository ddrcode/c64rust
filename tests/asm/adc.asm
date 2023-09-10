        JMP start

; stores number of correctly executed tests
result  !byte    0 
last_op !byte    0


; SUBROUTINES

; clears registers and C flag after each test 
next:
        INC result
        LDA #0
        LDX #0
        LDY #0
        CLC
        RTS

; PROGRAM

start:

test1:
        ; $50+$10=$60
        LDA #$50
        ADC #$10
        PHP
        BMI err_18
        BVS err_17
        BCS err_11
        CMP #$60
        BNE err_12
        JSR next

test2:
        ; $50+$50=$a0; NV set
        LDA #$50
        ADC #$50
        PHP
        BPL err_18
        BVC err_17
        BCS err_11
        CMP #$a0
        BNE err_12
        JSR next


test3:
        ; $50+$90=$e0; N set
        LDA #$50
        ADC #$90
        PHP
        BPL err_18
        BVS err_17
        BCS err_11
        CMP #$e0
        BNE err_12
        JSR next

test4:
        ; $50+$d0=$20; C set
        LDA #$50
        ADC #$d0
        PHP
        BMI err_18
        BVS err_17
        BCC err_11
        CMP #$20
        BNE err_12
        JSR next
        JMP test5

err_11: LDY #$11
        JMP err
err_12: LDY #$12
        JMP err
err_17: LDY #$17
        JMP err
err_18: LDY #$18
        JMP err
err:
        JMP end

test5:
        ; $d0+$10=$20; N set
        LDA #$d0
        ADC #$10
        PHP
        BPL err_18
        BVS err_17
        BCS err_11
        CMP #$e0
        BNE err_12
        JSR next

test6:
        ; $d0+$50=$20; C set
        LDA #$d0
        ADC #$50
        PHP
        BMI err_18
        BVS err_17
        BCC err_11
        CMP #$20
        BNE err_12
        JSR next

test7:
        ; $d0+$90=$60; CV set
        LDA #$d0
        ADC #$90
        PHP
        BMI err_18
        BVC err_17
        BCC err_11
        CMP #$60
        BNE err_12
        JSR next

test8:
        ; $d0+$d0=$a0; CM set
        LDA #$d0
        ADC #$d0
        PHP
        BPL err_18
        BVS err_17
        BCC err_11
        CMP #$a0
        BNE err_12
        JSR next

ok:
        LDY #0

end:
        LDA result
        PLP

    
