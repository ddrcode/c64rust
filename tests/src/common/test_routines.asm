; stores number of correctly executed tests
test_count  !byte    0 


; SUBROUTINES

; clears registers and C flag after each test 
next:
        INC test_count
        LDA #0
        LDX #0
        LDY #0
        CLC
        RTS

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

bmierr: BMI err_18
        RTS

bplerr: BPL err_18
        RTS

bvserr: BVS err_17
        RTS

bvcerr: BVC err_17
        RTS

bcserr: BCS err_11
        RTS

bccerr: BCC err_11
        RTS

bneerr: BNE err_12
        RTS

beqerr: BEQ err_12
        RTS

