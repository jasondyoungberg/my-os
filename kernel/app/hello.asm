[bits 64]
[org 0x1000]

; print message
    mov rax, 1
    mov rdi, 1
    mov rsi, message
    mov rdx, message.len
    syscall

; exit 0
    mov rax, 3
    mov rdi, 0
    syscall

jmp $

message: db `Hello, World!\n`
.len EQU $ - message
