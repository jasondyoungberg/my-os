[bits 64]
[org 0x1_0000]

mov r15, 10

main:

mov rax, 1
mov rdi, 1
mov rsi, msg
mov rdx, msg.len
syscall

mov rax, 24
syscall

dec r15
jnz main

mov rax, 1
mov rdi, 1
mov rsi, msg2
mov rdx, msg2.len
syscall

mov rax, 60
mov rdi, 0
syscall

jmp main

msg: db `Loopy!\n`
    .len EQU $ - msg

msg2: db `Done!\n`
    .len EQU $ - msg2
