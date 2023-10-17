;; Simple BCD add/sub test

        JMP start

!source "src/common/test_routines.asm"

start:

test1:  ;; 45 + 8 in BCD
        SED
        LDA #$45
        ADC #$08
        PHP
        CLD
        JSR bcserr
        CMP #$53
        JSR bneerr
        JSR next

test2:  ;; 99 + 1 in BCD
        SED
        LDA #$99
        ADC #$01
        PHP
        CLD
        JSR bccerr
        JSR bneerr
        JSR next

test3:  ;; 20 - 2 in BCD
        SEC
        SED
        LDA #$20
        SBC #$02
        PHP
        CLD
        JSR bccerr
        CMP #$18
        JSR bneerr
        JSR next

test4:  ;; 12 - 14 in BCD
        SEC
        SED
        LDA #$12
        SBC #$14
        PHP
        CLD
        JSR bcserr
        CMP #$98
        JSR bneerr
        JSR next

ok:
        LDY     #0

end:
        LDA     test_count
        PLP


