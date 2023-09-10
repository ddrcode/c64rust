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
        ; $50-$f0=$60
        LDA #$50
        SBC #$f0
        PHP
        BMI err_18
        BVS err_17
        BCS err_11
        CMP #$60
        BNE err_12
        JSR next

test2:
        ; $50+$b0=$a0; NV set
        LDA #$50
        SBC #$b0
        PHP
        BPL err_18
        BVC err_17
        BCS err_11
        CMP #$a0
        BNE err_12
        JSR next


test3:
        ; $50+$70=$e0; N set
        LDA #$50
        SBC #$70
        PHP
        BPL err_18
        BVS err_17
        BCS err_11
        CMP #$e0
        BNE err_12
        JSR next

test4:
        ; $50+$30=$20; C set
        LDA #$50
        SBC #$30
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
        ; $d0+$f0=$e0; N set
        LDA #$d0
        SBC #$f0
        PHP
        BPL err_18
        BVS err_17
        BCS err_11
        CMP #$e0
        BNE err_12
        JSR next

test6:
        ; $d0+$b0=$20; C set
        LDA #$d0
        SBC #$b0
        PHP
        BMI err_18
        BVS err_17
        BCC err_11
        CMP #$20
        BNE err_12
        JSR next

test7:
        ; $d0+$70=$60; CV set
        LDA #$d0
        SBC #$70
        PHP
        BMI err_18
        BVC err_17
        BCC err_11
        CMP #$60
        BNE err_12
        JSR next

test8:
        ; $d0+$30=$a0; CM set
        LDA #$d0
        SBC #$30
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

    
