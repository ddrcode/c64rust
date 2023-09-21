* = $e000

!raw "HelloOS by DDRcode"
!skip $ff

; --------------------------------------------------------------------
; GLOBAL ADDRESSESS 

reset_vector = $fce2
irq_vector = $ffb0
nmi_vector = $ffe0

screen_start = $0400
screen_end = $07e8

; --------------------------------------------------------------------
; PAGE ZERO VARIABLES

!addr var_a = $10                             ; addr of 16-bit general purpose variable (var A)
!addr var_b = $12                             ; addr of 16-bit general purpose variable (var B)
!addr var_c = $14                             ; addr of 16-bit general purpose variable (var B)
!addr irq_cnt = $16                           ; counts 0-50: IRQ cycles pers second
!addr cursor_y = $20                          ; current text line (cursor line)
!addr cursor_x = $21                          ; cursor-Y (0-39)
!addr cursor_line_addr = $22                  ; computed cursor address (0, y)

; --------------------------------------------------------------------
; ROM DATA

welcome_msg_line_0     !pet    "Welcome to HelloOS v0.1", 0
welcome_msg_line_1     !pet    "============================", 0
welcome_msg_line_2     !pet    "(proudly doing nothing, but the cursor", 0
welcome_msg_line_3     !pet    "should be blinking cheerfully)", 0

; --------------------------------------------------------------------
; MACROS

!macro set_word .addr, .lo, .hi {
    LDA #.lo
    STA .addr
    LDA #.hi
    STA .addr+1
}

!macro copy_word .from, .to {
    LDA .from
    STA .to
    LDA .from+1
    STA .to+1
}

!macro set_cursor .x, .y {
    LDA #.x
    STA cursor_x
    LDA #.y
    STA cursor_y
    JSR compute_cursor_line_addr
}

!macro zero .addr {
    LDA #0
    STA .addr
}

!macro println .txt {
    +set_word var_c, < .txt, > .txt
    JSR println
}

!macro println_at .txt, .x, .y {
    +set_cursor .x, .y
    +println .txt
}

; --------------------------------------------------------------------
; SUBROUTINES


;; Fills memory range with value from A
;; var_a (16-bit): beginning of the range (inclusive)
;; var_b (16-bit): end of the range (inclusive)
!zone sub_fill_mem
fill_mem:
        LDY var_a                       ; load lo-byte to y
        LDX #$0                         ; set lo-byte to 0
        STX var_a
        LDX var_a+1                     ; load hi-byte to x
.loop
        STA (var_a),Y                   ; set value on $hi00+y
        INY                             ; increase Y
        BNE +                           ; jump if Y is not 0
        INX                             ; else increment X and hi-byte
        INC var_a+1
+       CPX var_b+1                     ; compare X with end hi-byte
        BNE .loop                       ; if not equal - continue
        CPY var_b                       ; otherwise compare Y with end lo-byte
        BNE .loop                       ; continue if not equal
        RTS


;; Clears screen memory
!zone sub_cls
cls:
        +set_word var_a, < screen_start, > screen_start
        +set_word var_b, < screen_end, > screen_end
        LDA #$20                        ; space character
        JSR fill_mem
        RTS


!zone sub_show_cursor
show_cursor:
        LDA #$ff
        LDY cursor_x
        STA (cursor_line_addr), Y
        RTS


!zone sub_hide_cursor
hide_cursor:
        LDA #$20
        LDY cursor_x
        STA (cursor_line_addr), Y
        RTS


!zone 
;; Computes the screen address of the beggining of cursor Y line
;; and stores the result in cursor_line_variable
;; @uses var_a, var_b
compute_cursor_line_addr:
        +set_word var_a, < screen_start, > screen_start
        +set_word var_b, $00, $00
        LDY cursor_y                    ; set reg Y to cursor Y
.loop                                   ; and loop until Y is zero
        CPY #0                          ; if Y is 0 go to done 
        BEQ .done
        +set_word var_b, $28, $00       ; set var_b to $0028 (40 - line's lenght)
        JSR add                         ; add var_a and var_b
        DEY                             ; decrement Y
        JMP .loop
.done
        +copy_word var_a, cursor_line_addr
        RTS


!zone
;; print text at cursor position
;; FIXME it assumes that text never goes beyond screen
println:
        +copy_word cursor_line_addr, var_a
        LDX cursor_x 
        BEQ +                           ; jump if cursor X is 0
        +set_word var_b, 0, 0           ; else add cursor X to line addr
        STX var_b                       ; and store result in var_a
        JSR add
+       LDY #$0                         ; loop counter
.loop:
        LDA (var_c), Y                  ; load character to reg A
        BEQ .done                       ; finish if char 0
        STA (var_a), Y                  ; else print character on the screen
        INY                             ; increment counter
        JMP .loop
.done:
        INC cursor_y                    ; set cursor position to beginning of next line
        +zero cursor_x
        JSR compute_cursor_line_addr
        RTS


!zone sub_add
;; Adds var_a to var_b and stores the result in var_a
add:    CLC                             ; clear carry flag
        LDA var_a                       ; add low bytes of var a and b
        ADC var_b
        PHA                             ; store result on the stack
        LDA var_a+1                     ; add hi-bytes of a and b (with carry)
        ADC var_b+1	
        STA var_a+1
        PLA                             ; load lo-byte sum from the stack
        STA var_a                       ; and store in var_a low byte
        RTS


; --------------------------------------------------------------------
; INIT

!zone init_procedure 

; $fce2 is a starting procedure address of C64 official Kernal, so we use the same adress 
; to initialize the system (there is no particular reason for it other than fun or consistency)

* = reset_vector

init:
        SEI                             ; disable interrupts for init
        JSR cls                         ; Clear screen and print welcome message
        +println_at welcome_msg_line_0, 8, 1
        +println_at welcome_msg_line_1, 6, 2
        +println_at welcome_msg_line_2, 0, 4
        +println welcome_msg_line_3
        +set_cursor 0, 7
        CLI                             ; enable interrupts
.loop:                                  ; Cursor blinking loop
        LDA irq_cnt                     ; Show cursor on irq_cnt=0
        BNE .cont
        JSR show_cursor
.cont
        LDA #$19                        ; Hide cursor on irq_cnt=25
        CMP irq_cnt
        BNE .loop
        JSR hide_cursor

        JMP .loop


; --------------------------------------------------------------------
; INTERRUPTS HANDLING

!zone irq_handler

;; The interrupt procedure just increments irq_cnt variable
;; from 0 to 49 (as the CIA chips ticks at 50Hz or 60Hz
;; for PAL / NTSC systems)

* = irq_vector
    PHA                                 ; Push A
    TXA                                 ; Copy X to A and push
    PHA
    TYA                                 ; Copy Y to A and push
    PHA
    TSX                                 ; Copy stack pointer
    LDA $0104,X                         ; Load stacked status register
    AND #$10                            ; Mask BRK flag
    BEQ +                               ; Branch if not BRK
    JMP .end                            ; Else do this (BRK case)

+   INC irq_cnt
    LDA #$32
    CMP irq_cnt
    BNE .end
    LDA #$0
    STA irq_cnt
.end
    PLA                                 ; Pull and copy to Y
    TAY
    PLA                                 ; Pull and copy to X
    TAX
    PLA                                 ; Pull A
    RTI                                 ; Resume program execution

!zone nmi_handler
* = nmi_vector
    RTI


; --------------------------------------------------------------------

!zone hardware_vectors

* = $fffa

nmi !word nmi_vector
rst !word reset_vector
irq !word irq_vector
