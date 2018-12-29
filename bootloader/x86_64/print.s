; Function print:
; Idea: while (string[i] != 0) { print string[i]; i++ } but in asm
print:
    pusha

    ; The comparison end when found null byte `0x00`
    start:
        ; 'bx' is the base address for the string
        mov al, [bx]
        cmp al, 0
        je done

        ; Printing with help of the BIOS
        mov ah, 0x0E ; TTY mode
        ; 'al' already contains the char to be printed
        int 0x10

        ; Increment and repeat the loop
        add bx, 1
        jmp start

    done:
        popa
        ret

; Function print_nl
; Prints a newline char
print_nl:
    pusha

    mov ah, 0x0E
    mov al, 0x0A ; newline char
    int 0x10
    mov al, 0x0D ; carriage return
    int 0x10

    popa
    ret


; Function print_hex
; Prints in hexadecimal
print_hex:
    pusha

    ; Index variable
    mov cx, 0

    ; Strategy: get the last char of 'dx', then convert to ASCII
    ; Numeric ASCII values: '0' (ASCII 0x30) to '9' (0x39), so just add 0x30 to byte N.
    ; For alphabetic characters A-F: 'A' (ASCII 0x41) to 'F' (0x46) we'll add 0x40
    ; Then, move the ASCII byte to the correct position on the resulting string
    loop_hex:
        cmp cx, 4
        je end

        ; 1. Convert last char of dx to ascii
        ; 'ax' as working register
        mov ax, dx
        and ax, 0x000F
        ; 0x1234 -> 0x0004 by masking first three to zeros
        add al, 0x30
        ; If > 9, add extra 8 to represent 'A' to 'F'
        cmp al, 0x39
        jle step2
        ; 'A' is ASCII 65 instead of 58, so 65-58=7
        add al, 7

    step2:
        ; 2. Get the correct position of the string to place our ASCII char
        ; bx <- base address + string length - index of char
        mov bx, HEX_OUT + 5 ; base + length
        sub bx, cx ; ans - index of char
        ; copy the ASCII char on 'al' to the position pointed by 'bx'
        mov [bx], al
        ror dx, 4 ; 0x1234 -> 0x4123 -> 0x3412 -> 0x2341 -> 0x1234

        ; Increment and repeat the loop
        add cx, 1
        jmp loop_hex

    end:
        ; Prepare the parameter and call the function
        ; Remember that print receives parameters in 'bx'
        mov bx, HEX_OUT
        call print

        popa
        ret

HEX_OUT:
    db '0x0000',0 ; reserve memory for our new string
