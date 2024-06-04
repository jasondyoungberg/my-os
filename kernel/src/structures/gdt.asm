section .text

global gdt_load
gdt_load:
    mov [rel gdtr], di
    mov [rel gdtr+2], rsi
    lgdt [rel gdtr]
    push 0x08
    lea rax, [rel .reload]
    push rax
    retfq
.reload:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    ret

section .data

gdtr:
    dw 0
    dq 0
