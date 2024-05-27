[bits 64]
[org 0x1000]


mov rax, 1
mov rdi, 1
mov rsi, hello
mov rdx, hello.len
syscall

mov rdi, 1
mov rsi, msg
mov rdx, msg.len
cycle:
mov rcx, 10_000_000
loop $

mov rax, 1
syscall

jmp cycle

hello: db "Hello, World!"
.len EQU $ - hello
msg: db "I'm a Thread!"
.len EQU $ - msg
