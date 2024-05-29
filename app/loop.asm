[bits 64]
[org 0x1000]

cycle:

; print message
    mov rax, 1
    mov rdi, 1
    mov rsi, message
    mov rdx, message.len
    syscall

mov rcx, 500_000_000
loop $

jmp cycle

message: db `I'm a busy loop!\n`
.len EQU $ - message
