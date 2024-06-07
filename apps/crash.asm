[bits 64]
[org 0x1_0000]

main:

mov rax, 1
mov rdi, 1
mov rsi, msg
mov rdx, msg.len
syscall

mov rax, [0x123456789]

jmp main

msg: db `I'm gonna crash!\n`
    .len EQU $ - msg
