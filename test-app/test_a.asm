[bits 64]
[org 0x1000]

mov rax, 2
mov rdi, msg
mov rsi, msg.len
syscall
mov rax, 0
mov rdi, 0
syscall
jmp $


msg: db "Hello from userland!"
    .len EQU $ - msg
