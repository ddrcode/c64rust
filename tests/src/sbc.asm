        JMP start

!source "src/common/test_routines.asm"

; PROGRAM

start:

test1:
        ; $50-$f0=$60
        LDA #$50
        SEC
        SBC #$f0
        PHP
        JSR bmierr
        JSR bvserr
        JSR bcserr
        CMP #$60
        JSR bneerr
        JSR next

test2:
        ; $50+$b0=$a0; NV set
        LDA #$50
        SEC
        SBC #$b0
        PHP
        JSR bplerr
        JSR bvcerr
        JSR bcserr
        CMP #$a0
        JSR bneerr
        JSR next


test3:
        ; $50+$70=$e0; N set
        LDA #$50
        SEC
        SBC #$70
        PHP
        JSR bplerr
        JSR bvserr
        JSR bcserr
        CMP #$e0
        JSR bneerr
        JSR next

test4:
        ; $50+$30=$20; C set
        LDA #$50
        SEC
        SBC #$30
        PHP
        JSR bmierr
        JSR bvserr
        JSR bccerr
        CMP #$20
        JSR bneerr
        JSR next
        JMP test5

test5:
        ; $d0+$f0=$e0; N set
        LDA #$d0
        SEC
        SBC #$f0
        PHP
        JSR bplerr
        JSR bvserr
        JSR bcserr
        CMP #$e0
        JSR bneerr
        JSR next

test6:
        ; $d0+$b0=$20; C set
        LDA #$d0
        SEC
        SBC #$b0
        PHP
        JSR bmierr
        JSR bvserr
        JSR bccerr
        CMP #$20
        JSR bneerr
        JSR next

test7:
        ; $d0+$70=$60; CV set
        LDA #$d0
        SEC
        SBC #$70
        PHP
        JSR bmierr
        JSR bvcerr
        JSR bccerr
        CMP #$60
        JSR bneerr
        JSR next

test8:
        ; $d0+$30=$a0; CM set
        LDA #$d0
        SEC
        SBC #$30
        PHP
        JSR bplerr
        JSR bvserr
        JSR bccerr
        CMP #$a0
        JSR bneerr
        JSR next

ok:
        LDY #0

end:
        LDA test_count
        PLP

    
