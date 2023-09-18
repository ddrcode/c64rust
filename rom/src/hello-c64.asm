* = $e000

; --------------------------------------------------------------------
; GLOBAL ADDRESSESS 

reset_vector = $fce2
irq_vector = $ffc0
nmi_vector = $ffe0

screen_start = $0400
screen_end = $07e8

; --------------------------------------------------------------------
; PAGE ZERO VARIABLES

!addr var_a = $10                             ; addr of 16-bit general purpose variable (var A)
!addr var_b = $12                             ; addr of 16-bit general purpose variable (var B)
!addr var_c = $14                             ; addr of 16-bit general purpose variable (var B)
!addr irq_cnt = $16                           ; counts 0-50: IRQ cycles pers second
!addr curr_line = $20                         ; current text line (cursor line)
!addr curr_column = $21                       ; cursor-Y (0-39)

; --------------------------------------------------------------------
; ROM DATA

welcome_msg     !pet "Hello C64Rust!", 0
default_cursor  !scr "A", 1, 0

; --------------------------------------------------------------------
; MACROS

!macro set_word .addr, .lo, .hi {
    LDA #.lo
    STA .addr
    LDA #.hi
    STA .addr+1
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

!zone get_cursor_addr
;; Computes an address of screen memory at cursor position
;; Inputs: none
;; Outputs: var_c
cursor_addr:
        +set_word var_a, $00, $04
        +set_word var_b, $00, $00
        LDY curr_line                   ; set reg Y to cursor Y
.loop                                   ; and loop until Y is zero
        CPY #0                          ; if Y is 0 go to done 
        BEQ .done
        +set_word var_b, $28, $00       ; set var_b to $0028 (40 - line's lenght)
        JSR add                         ; add var_a and var_b
        LDA var_c                       ; copy var_c to var_a 
        STA var_a
        LDA var_c+1
        STA var_a+1
        DEY                             ; decrement Y
        JMP .loop
.done
        LDA curr_column                 ; set reg A to cursor x
        STA var_b                       ; set var_b to $00cur_y
        LDA #0
        STA var_b+1
        JSR add                         ; add cur_x to result   
        RTS

!zone sub_print_text
print:
        ; JSR cursor_addr
        LDY #$0

.loop:
        LDA (var_a), Y
        CMP #0
        BEQ .done
        STA (var_c), Y 
        INY
        JMP .loop
.done:
        INC curr_line
        RTS


!zone sub_add
add:    CLC				        ; clear carry
        LDA var_a
        ADC var_b
        STA var_c			    ; store sum of lo-bytes
        LDA var_a+1
        ADC var_b+1	    		; add the hi-byes using carry from
        STA var_c+1	    	; the previous calculation
        RTS

; --------------------------------------------------------------------
; INIT

!zone init_procedure 

; $fce2 is a starting procedure address of C64 official Kernal, so we use the same adress 
; to initialize the system (there is no particular reason for it other than fun or consistency)
* = reset_vector

init:
        LDA #0                          ; Set cursor line and column to 0
        STA curr_line           
        LDA #0                          ; Set cursor line and column to 0
        STA curr_column           
        JSR cls                         ; Clear screen
        LDX #$0

        JSR cursor_addr
        +set_word var_a, < welcome_msg, > welcome_msg
        JSR print

.loop:
;         LDA irq_cnt
;         BNE .cont
;         JSR show_cursor
; .cont
;         LDA #$19
;         CMP irq_cnt
;         BNE .loop
;         JSR hide_cursor
;
        NOP
        JMP .loop

; * = reset_vector


; --------------------------------------------------------------------
; INTERRUPTS HANDLING

!zone irq_handler

* = irq_vector
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

!zone nmi_handler
* = nmi_vector
    RTI

; --------------------------------------------------------------------

!zone hardware_vectors

* = $fffa

nmi !word nmi_vector
rst !word reset_vector
irq !word irq_vector


