PORTB = $6000
PORTA = $6001
DDRB = $6002
DDRA = $6003

E  = %10000000
RW = %01000000
RS = %00100000

    .org $8000
reset:
    lda #%11111111 ; Set all pins on port B to output
    sta DDRB

    lda #%11100000 ; Set top 3 pins on port A to output
    sta DDRA

    lda #%00111000 ; Set 8-bit mode; 2-line display; 5x8 font 
    sta PORTB

    lda #0 ; clear RS/RW/E bits for display
    sta PORTA

    lda #E ; Toggle enable bit to send instruction
    sta PORTA

    lda #0 ; clear RS/RW/E bits for display
    sta PORTA

    lda #%00001110 ; Display on, cursor on, cursor blink off
    sta PORTB

    lda #0 ; clear RS/RW/E bits for display
    sta PORTA

    lda #E ; Toggle enable bit to send instruction
    sta PORTA

    lda #0 ; clear RS/RW/E bits for display
    sta PORTA

    lda #%00000110 ; Set entry mode to increment on, shift off
    sta PORTB
    
    lda #0 ; clear RS/RW/E bits for display
    sta PORTA

    lda #E ; Toggle enable bit to send instruction
    sta PORTA

    lda #0 ; clear RS/RW/E bits for display
    sta PORTA

    lda #"H"
    sta PORTB

    lda #RS ; Set register select bit
    sta PORTA

    lda #(RS | E) ; Toggle enable and register select bit to send instruction
    sta PORTA

    lda #RS ; Set register select bit
    sta PORTA

    lda #"e"
    sta PORTB

    lda #RS ; Set register select bit
    sta PORTA

    lda #(RS | E) ; Toggle enable and register select bit to send instruction
    sta PORTA

    lda #RS ; Set register select bit
    sta PORTA

    lda #"l"
    sta PORTB

    lda #RS ; Set register select bit
    sta PORTA

    lda #(RS | E) ; Toggle enable and register select bit to send instruction
    sta PORTA

    lda #RS ; Set register select bit
    sta PORTA

    lda #"l"
    sta PORTB

    lda #RS ; Set register select bit
    sta PORTA

    lda #(RS | E) ; Toggle enable and register select bit to send instruction
    sta PORTA

    lda #RS ; Set register select bit
    sta PORTA

    lda #"o"
    sta PORTB

    lda #RS ; Set register select bit
    sta PORTA

    lda #(RS | E) ; Toggle enable and register select bit to send instruction
    sta PORTA

    lda #RS ; Set register select bit
    sta PORTA

    lda #","
    sta PORTB

    lda #RS ; Set register select bit
    sta PORTA

    lda #(RS | E) ; Toggle enable and register select bit to send instruction
    sta PORTA

    lda #RS ; Set register select bit
    sta PORTA

    lda #" "
    sta PORTB

    lda #RS ; Set register select bit
    sta PORTA

    lda #(RS | E) ; Toggle enable and register select bit to send instruction
    sta PORTA

    lda #RS ; Set register select bit
    sta PORTA

    lda #"w"
    sta PORTB

    lda #RS ; Set register select bit
    sta PORTA

    lda #(RS | E) ; Toggle enable and register select bit to send instruction
    sta PORTA

    lda #RS ; Set register select bit
    sta PORTA

    lda #"o"
    sta PORTB

    lda #RS ; Set register select bit
    sta PORTA

    lda #(RS | E) ; Toggle enable and register select bit to send instruction
    sta PORTA

    lda #RS ; Set register select bit
    sta PORTA

    lda #"r"
    sta PORTB

    lda #RS ; Set register select bit
    sta PORTA

    lda #(RS | E) ; Toggle enable and register select bit to send instruction
    sta PORTA

    lda #RS ; Set register select bit
    sta PORTA

    lda #"l"
    sta PORTB

    lda #RS ; Set register select bit
    sta PORTA

    lda #(RS | E) ; Toggle enable and register select bit to send instruction
    sta PORTA

    lda #RS ; Set register select bit
    sta PORTA

    lda #"d"
    sta PORTB

    lda #RS ; Set register select bit
    sta PORTA

    lda #(RS | E) ; Toggle enable and register select bit to send instruction
    sta PORTA

    lda #RS ; Set register select bit
    sta PORTA

    lda #"!"
    sta PORTB

    lda #RS ; Set register select bit
    sta PORTA

    lda #(RS | E) ; Toggle enable and register select bit to send instruction
    sta PORTA

    lda #RS ; Set register select bit
    sta PORTA

loop:
    jmp loop

    .org $fffc
    .word reset
    .word $0000