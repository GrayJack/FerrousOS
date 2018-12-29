[bits 32]
; Define calling point. Must have same name as kernel.c 'main' function
[extern _start]
; Calls the C function. The linker will know where it is placed in memory
call _start
jmp $
