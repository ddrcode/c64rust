; 16-bit addition and subtraction simple example by FMan/Tropyx

    JMP test

!source "src/common/test_routines.asm"

                        
val1            !word   $0430, $0100, $0000, $00f1
val2            !word   $00a2, $2345, $0000, $0092
sums            !word   $04d2, $2445, $0000, $0183  ; tests 1,3,5,7
subs            !word   $038e, $ddbb, $0000, $005f  ; tests 2,4,6,8

num1            !word   0
num2            !word   0
result          !word   0
expected_sum    !word   0
expected_sub    !word   0

sum_count       !byte   0

; adds numbers 1 and 2, writes result to separate location

add	    clc				        ; clear carry
        lda num1
        adc num2
        sta result			    ; store sum of lo-bytes
        lda num1+1
        adc num2+1	    		; add the hi-byes using carry from
        sta result+1	    	; the previous calculation
        rts

sub	    sec				        ; set carry for borrow purpose
        lda num1
        sbc num2			    ; perform subtraction on lo-bytes
        sta result
        lda num1+1			    ; load hi-bytes
        sbc num2+1			    ; perform subtraction (with borrow - C flag - from the previous sub)
        sta result+1
        rts

get_nums                        ; puts values from the arrays (val1, val2, sums, subs)
        lda val1,Y              ; into variables
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
        lda test_count          ; check whether the counter is equal num of tests (lame!
        cmp #$4                 ; (I know - so lame!)
        beq ok                  ; if so - jump to ok

        asl                     ; double test_count in A
        tay
        jsr get_nums

        jsr add                 ; perform addition

        lda result              ; test the lo byte
        cmp expected_sum
        php
        jsr bneerr

        lda result+1            ; test the hi byte
        cmp expected_sum+1
        jsr bneerr
        inc sum_count

        jsr sub                 ; perform subtraction

        lda result              ; test the lo byte
        cmp expected_sub
        php
        jsr bneerr

        lda result+1            ; test the hi byte
        cmp expected_sub+1
        jsr bneerr

        jsr next                ; clean registers and inc test_count
        jmp test                ; loop
ok
        ldy #$0
end
        clc
        lda test_count
        adc sum_count
        plp
