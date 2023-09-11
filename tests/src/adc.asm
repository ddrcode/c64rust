        JMP start

!source "src/common/test_routines.asm"

; PROGRAM

start:

test1:
        ; $50+$10=$60
        LDA #$50
        ADC #$10
        PHP
        JSR bmierr
        JSR bvserr
        JSR bcserr
        CMP #$60
        JSR bneerr
        JSR next

test2:
        ; $50+$50=$a0; NV set
        LDA #$50
        ADC #$50
        PHP
        JSR bplerr
        JSR bvcerr
        JSR bcserr
        CMP #$a0
        JSR bneerr
        JSR next


test3:
        ; $50+$90=$e0; N set
        LDA #$50
        ADC #$90
        PHP
        JSR bplerr
        JSR bvserr
        JSR bcserr
        CMP #$e0
        JSR bneerr
        JSR next

test4:
        ; $50+$d0=$20; C set
        LDA #$50
        ADC #$d0
        PHP
        JSR bmierr
        JSR bvserr
        JSR bccerr
        CMP #$20
        JSR bneerr
        JSR next

test5:
        ; $d0+$10=$20; N set
        LDA #$d0
        ADC #$10
        PHP
        JSR bplerr
        JSR bvserr
        JSR bcserr
        CMP #$e0
        JSR bneerr
        JSR next

test6:
        ; $d0+$50=$20; C set
        LDA #$d0
        ADC #$50
        PHP
        JSR bmierr
        JSR bvserr
        JSR bccerr
        CMP #$20
        JSR bneerr
        JSR next

test7:
        ; $d0+$90=$60; CV set
        LDA #$d0
        ADC #$90
        PHP
        JSR bmierr
        JSR bvcerr
        JSR bccerr
        CMP #$60
        JSR bneerr
        JSR next

test8:
        ; $d0+$d0=$a0; CM set
        LDA #$d0
        ADC #$d0
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

