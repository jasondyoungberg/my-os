[bits 64]
[org 0x1_0000]

main:

mov rax, 0x1
mov rdi, 0x1
mov rsi, msg
mov rdx, msg.len
syscall

mov rax, 0x60
mov rdi, 0
syscall

jmp main

msg: db `Hello, World!\n`
    .len EQU $ - msg
