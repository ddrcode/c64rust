; 16-bit addition and subtraction simple example by FMan/Tropyx

    JMP test

!source "src/common/test_routines.asm"

val1            !word   $0430, $0100, $0000, $00f1
val2            !word   $00a2, $2345, $0000, $0092
sums            !word   $04d2, $2445, $0000, $0183
subs            !word   $038e, $ddbb, $0000, $005f

num1            !word   0
num2            !word   0
result          !word   0
expected_sum    !word   0
expected_sub    !word   0

sum_count       !byte   0

; adds numbers 1 and 2, writes result to separate location

add	    clc				; clear carry
        lda num1
        adc num2
        sta result			; store sum of LSBs
        lda num1+1
        adc num2+1			; add the MSBs using carry from
        sta result+1		; the previous calculation
        rts

; subtracts number 2 from number 1 and writes result out

sub	    sec				; set carry for borrow purpose
        lda num1
        sbc num2			; perform subtraction on the LSBs
        sta result
        lda num1+1			; do the same for the MSBs, with carry
        sbc num2+1			; set according to the previous result
        sta result+1
        rts

get_nums
        lda val1,Y
        sta num1
        lda val1+1,Y
        sta num1+1

        lda val2,Y
        sta num2
        lda val2+1,Y
        sta num2+1

        lda sums,Y
        sta expected_sum
        lda sums+1,Y
        sta expected_sum+1

        lda subs,Y
        sta expected_sub
        lda subs+1,Y
        sta expected_sub+1

        rts

test
        lda test_count
        cmp #$4
        beq ok

        asl
        tay
        jsr get_nums

        jsr add

        lda result
        cmp expected_sum
        php
        jsr bneerr

        lda result+1
        cmp expected_sum+1
        jsr bneerr
        inc sum_count

        jsr sub

        lda result
        cmp expected_sub
        php
        JSR bneerr

        lda result+1
        cmp expected_sub+1
        jsr bneerr

        jsr next
        jmp test
ok
        ldy #$0
end
        clc
        lda test_count
        adc sum_count
        plp
