[bits 64]
[org 0x1000]

; print message
    push 1
    push 1
    push message2
    push message2.len
    mov rax, 1
    mov rdi, 1
    mov rsi, message
    mov rdx, message.len
    syscall
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

message: db `Stack test:`
.len EQU $ - message
message2: db ` Passed\n`
.len EQU $ - message2
