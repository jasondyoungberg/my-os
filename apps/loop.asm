[bits 64]
[org 0x1_0000]

main:

mov rax, 1
mov rdi, 1
mov rsi, msg
mov rdx, msg.len
syscall

mov rax, 24
syscall

jmp main

msg: db `Loopy!\n`
    .len EQU $ - msg
