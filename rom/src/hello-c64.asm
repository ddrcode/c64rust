* = $e000

; --------------------------------------------------------------------
; GLOBAL ADDRESSESS 

reset_vector = $fce2
irq_vector = $fe00
nmi_vector = $ff00

screen_start = $0400
screen_end = $07e8

; --------------------------------------------------------------------
; PAGE ZERO VARIABLES

var_a = $10                             ; addr of 16-bit general purpose variable (var A)
var_b = $12                             ; addr of 16-bit general purpose variable (var B)
irq_cnt = $14                           ; counts 0-50: IRQ cycles pers second

; --------------------------------------------------------------------
; ROM DATA

welcome_msg     !pet "Hello C64Rust!", 0
default_cursor  !scr "A", 1, 0

; --------------------------------------------------------------------
; MACROS

!macro set_word .addr, .lo, .hi {
    LDY #.lo
    LDX #.hi
    STY .addr
    STX .addr+1
}

; --------------------------------------------------------------------
; SUBROUTINES

;; Fills memory range with value from A
;; $10 (16-bit): beginning of the range (inclusive)
;; $12 (16-bit): end of the range (inclusive)
!zone sub_fill_mem
fill_mem:
        LDY var_a                       ; load lo-byte to y
        LDX #$0                         ; set lo-byte to 0
        STX var_a
        LDX var_a+1                     ; load hi-byte to x
.loop
        STA (var_a),Y                   ; set value on $hi00+y
        INY                             ; increase Y
        BNE .cont                       ; if not 0 go to .cont
        INX                             ; otherwise increment X and hi-byte
        INC var_a+1
.cont
        CPX var_b+1                     ; compare X with end hi-byte
        BNE .loop                       ; if not equal - continue
        CPY var_b                       ; otherwise compare Y with end lo-byte
        BNE .loop                       ; continue if not equal
.end
        RTS

;; Clears screen memory
!zone sub_cls
cls:
        +set_word var_a, $00, $04
        +set_word var_b, $e8, $07
        LDA #$20
        JSR fill_mem
        RTS


!zone sub_show_cursor
show_cursor:
        LDA #$ff
        STA screen_start+40             ; fixed position at this stage :-)
        RTS

!zone sub_hide_cursor
hide_cursor:
        LDA #$20
        STA screen_start+40
        RTS

; --------------------------------------------------------------------
; INIT

!zone init_procedure 

; $fce2 is a starting procedure address of C64 official Kernal, so we use the same adress 
; to initialize the system (there is no particular reason for it other than fun or consistency)
* = $fce2

init:
        JSR cls
        LDX #$0

.loop:
        LDA welcome_msg, X
        CMP #0
        BEQ .done
        STA screen_start, X 
        INX
        JMP .loop
.done:
        LDA irq_cnt
        BNE .cont
        JSR show_cursor
.cont
        LDA #$19
        CMP irq_cnt
        BNE .done
        JSR hide_cursor

        NOP
        JMP .done

; --------------------------------------------------------------------
; INTERRUPTS HANDLING

!zone interrupts

* = $ffe0
    PHA
    INC irq_cnt
    LDA #$32
    CMP irq_cnt
    BNE .end
    LDA  #$0
    STA irq_cnt
.end
    PLA
    RTI

* = $fff0
    RTI

; --------------------------------------------------------------------

!zone hardware_vectors

* = $fffa

nmi !word $fff0
rst !word $fce2
irq !word $ffe0


