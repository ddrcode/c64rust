; Tests 6502 CMP instruction for all address modes
; 
; Results:
;    A register:
;       Test passed: Register A set to $42
;       Test failed: Register A set to a branch op number that behaved incorrectly
;            (result variable is being incremented after each branch)
;    X register: the last tested CMP opcode (i.e. $dd for CMP Absolute,X)
;    Y register: always 0
;    P register: always 00110010 (consider error if not)
;
; Algorithm
; 1. The program contains a few CMP tests - one per adress mode
; 2. Each test runs a CMP instruction and then contains three branch operations
;    for Z, N and C flags respectively. The expected behavior is that program
;    continues to the end without branching. Branching means your emulator behaves
;    incorrectly (each branch always takes to 'err' section in the code)
; 3. At the beginning of every test the code goins to 'clear' subroutine that
;    clears A, X and Y registers
; 4. After each test (branching) the program calls 'inc_score' soubroutine that
;    increments result variable. In case of error, the value of that variable will bo
;    loaded to a A register to help identify where the error occured. 
;
; Assembling
;   Assembler: ACME (https://github.com/meonwax/acme)
;   Command: acme -f plain --cpu 6502 -o cmp.p cmp.a
; 
; Author: David de Rosier

; * = $0600

        JMP start


; SUBROUTINES

!source "src/common/test_routines.asm"


; PROGRAM

start:
        LDA #$80
        STA $f0



; Tests CMP for immediate address mode
; opcode: $C9
imd:
        LDA #$66
        CMP #$66
        PHP
        
        JSR bneerr
        JSR bmierr
        JSR bccerr
        
        JSR next      ; result = 3



zp:                        ; test for zero-page, equal
        LDA #$80
        CMP $f0
        PHP

        JSR bneerr
        JSR bmierr
        JSR bccerr
        JSR next      ; result = 6



zpx:
        LDA #$f0
        STA $f0

        LDA #$01
        LDX #$03
        CMP ($ed,X)
        PHP

        JSR beqerr
        JSR bmierr
        JSR bcserr
        JSR next



aby:
        LDA #$04
        STA $f0
        LDA #$95
        LDY #$1
        
        CMP $ef,Y
        PHP

        JSR beqerr
        JSR bplerr
        JSR bccerr
        JSR next

ok:
        LDY #0

end:
        LDA test_count
        PLP

