[bits 16]
switch_to_pm:
    ; 1. disable interrupts
    cli
    ; 2. load the GDT descriptor
    lgdt [gdt_descriptor]
    mov eax, cr0
    ; 3. set 32-bit mode bit in cr0
    or eax, 0x1
    mov cr0, eax
    ; 4. far jump by using a different segment
    jmp CODE_SEG:init_pm

[bits 32]
init_pm: ; We are now using 32-bit instructions

    ; 5. update the segment registers
    mov ax, DATA_SEG
    mov ds, ax
    mov ss, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; 6. update the stack right at the top of the free space
    mov ebp, 0x90000
    mov esp, ebp

    ; 7. Call a well-known label with useful code
    call BEGIN_PM
