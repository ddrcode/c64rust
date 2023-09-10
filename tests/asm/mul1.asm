;; Computes 10*3 multiplication
;; Modified version of an example from here
;; see https://codeburst.io/lets-write-some-harder-assembly-language-code-c7860dcceba

        JMP     start


mpr     !byte   $0a
mpd     !byte   $3
result  !word   0
tmp     !byte   0


start   LDX     #$8      ; x is a counter
mult    LSR     mpr      ; shift mpr right - pushing a bit into C
        BCC     noadd    ; test carry bit
        LDA     result   ; load A with low part of result
        CLC
        ADC     mpd      ; add mpd to res
        STA     result   ; save result
        LDA     result+1 ; add rest off shifted mpd
        ADC     tmp
        STA     result+1
noadd   ASL     mpd      ; shift mpd left, ready for next "loop"
        ROL     tmp      ; save bit from mpd into tmp
        DEX              ; decrement counter
        BNE     mult     ; go again if counter 0
end     LDA     result   ; store RESULT in A

