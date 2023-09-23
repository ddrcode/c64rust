;; Keyboard scanning functionality

!zone

!addr .port_a       = $dc00
!addr .port_b       = $dc01
!addr .scan_res     = $50
!addr .key_cnt      = $58


keyboard_scan:

;; Quickest possible scan - no loop, no jumps
;; Credits: https://codebase64.org/doku.php?id=base:scanning_the_keyboard_the_correct_and_non_kernal_way
.scan:
        LDA #%11111110
        STA .port_a
        LDX .port_b
        STX .scan_res+7

        SEC
        ROL
        STA .port_a
        LDX .port_b
        STX .scan_res+6

        ROL
        STA .port_a
        LDX .port_b
        STX .scan_res+5

        ROL
        STA .port_a
        LDX .port_b
        STX .scan_res+4

        ROL
        STA .port_a
        LDX .port_b
        STX .scan_res+3

        ROL
        STA .port_a
        LDX .port_b
        STX .scan_res+2

        ROL
        STA .port_a
        LDX .port_b
        STX .scan_res+1

        ROL
        STA .port_a
        LDX .port_b
        STX .scan_res

; My invention starts here :-)

        LDA #0                              ; zero number of keys found
        STA .key_cnt
        LDX #8

.check        
        LDA .scan_res-1, X
        CMP #$ff                            ; $ff = no key found
        BEQ +                               ; key found
        
        INC .key_cnt

+       DEX
        BNE .check


RTS
