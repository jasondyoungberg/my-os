[bits 64]
[org 0x1000]

cycle:

; print message
    mov rax, 1
    mov rdi, 1
    mov rsi, message
    mov rdx, message.len
    syscall

; sleep 0
    mov rax, 4
    mov rdi, 0
    syscall

jmp cycle

message: db `I'm a yeild loop!\n`
.len EQU $ - message
