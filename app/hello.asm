[bits 64]
[org 0x1000]

; print message
    mov rax, 1
    mov rdi, 1
    mov rsi, message
    mov rdx, message.len
    syscall

cycle:

; exit 0
    mov rax, 60
    mov rdi, 0
    syscall

; print message 2
    mov rax, 1
    mov rdi, 2
    mov rsi, message2
    mov rdx, message2.len
    syscall

jmp cycle

message: db `Hello, World!\n`
.len EQU $ - message

message2: db `I didn't exit!\n`
.len EQU $ - message2
