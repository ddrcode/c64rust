; ACME 0.94.4

        JMP start

!source "src/common/test_routines.asm"


!macro assert .res, .err_n, .err_z, .err_c {
        JSR .err_n
        JSR .err_z
        JSR .err_c
        CMP #.res
        JSR bneerr
        JSR next
}

!macro asl_test .val, .res, .err_n, .err_z, .err_c {
        LDA #.val
        ASL
        +assert .res, .err_n, .err_z, .err_c 
}

!macro lsr_test .val, .res, .err_n, .err_z, .err_c {
        LDA #.val
        LSR
        +assert .res, .err_n, .err_z, .err_c 
}

!macro rol_test .val, .res, .err_n, .err_z, .err_c {
        LDA #.val
        ROL
        +assert .res, .err_n, .err_z, .err_c 
}

!macro ror_test .val, .res, .err_n, .err_z, .err_c {
        LDA #.val
        ROR
        +assert .res, .err_n, .err_z, .err_c 
}

; PROGRAM

start:

    +asl_test $03, $06, bmierr, beqerr, bcserr
    +asl_test $ff, $fe, bplerr, beqerr, bccerr
    +asl_test $00, $00, bmierr, bneerr, bcserr

    +lsr_test $03, $01, bmierr, beqerr, bccerr
    +lsr_test $ff, $7f, bmierr, beqerr, bccerr
    +lsr_test $00, $00, bmierr, bneerr, bcserr

    +rol_test $03, $06, bmierr, beqerr, bcserr
    +rol_test $ff, $fe, bplerr, beqerr, bccerr
    +rol_test $00, $00, bmierr, bneerr, bcserr

    +ror_test $03, $01, bmierr, beqerr, bccerr
    +ror_test $ff, $7f, bmierr, beqerr, bccerr
    +ror_test $00, $00, bmierr, bneerr, bcserr

ok:
        LDY #0

end:
        LDA test_count
        PLP

