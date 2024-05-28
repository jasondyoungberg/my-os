[bits 64]
[org 0x1000]

; print message
    push 1
    push 1
    push message
    push message.len
    pop rdx
    pop rsi
    pop rdi
    pop rax
    syscall

cycle:

; exit 0
    mov rax, 60
    mov rdi, 0
    syscall
jmp cycle

message: db `Stack test.\n`
.len EQU $ - message
