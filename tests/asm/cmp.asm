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

; stores number of correctly executed tests
result  !byte    0 
last_op !byte    0


; SUBROUTINES

; clears registers and C flag after each test 
clear:
        LDA #0
        LDX #0
        LDY #0
        CLC
        RTS

; increments result variable after each test
; preserving status flags (as they are needed for following tests)
inc_score:
        PHP
        INC result
        PLP
        RTS


; PROGRAM

start:
        LDA #$80
        STA $f0



; Tests CMP for immediate address mode
; opcode: $C9
imd:
        LDA #$c9
        STA last_op

        JSR clear

        LDA #$66
        CMP #$66
        
        BNE err
        JSR inc_score      ; result = 1
        
        BMI err
        JSR inc_score      ; result = 2
        
        BCC err
        JSR inc_score      ; result = 3



zp:                        ; test for zero-page, equal
        LDA #$c5
        STA last_op

        JSR clear

        LDA #$80
        CMP $f0
        
        BNE err
        JSR inc_score      ; result = 4
        
        BMI err
        JSR inc_score      ; result = 5
        
        BCC err
        JSR inc_score      ; result = 6



zpx:
        LDA #$d5
        STA last_op

        LDA #$f0
        STA $f0
        JSR clear

        LDA #$01
        LDX #$03
        CMP ($ed,X)

        BEQ err
        JSR inc_score
        
        BMI err
        JSR inc_score
        
        BCS err
        JSR inc_score



aby:
        LDA #$04
        STA $f0
        JSR clear
        LDA #$95
        LDY #$1
        CMP $ef,Y
        BEQ err
        JSR inc_score
        BPL err
        JSR inc_score
        BCC err
        JSR inc_score

ok:
        LDA #$42
        JMP end

err:
        LDA result

end:
        LDX last_op
        LDY #0
        CLC

