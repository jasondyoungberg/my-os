[bits 64]
[org 0x1000]


mov rax, 1
mov rdi, hello
mov rsi, hello.len
syscall

mov rdi, msg
mov rsi, msg.len
cycle:
mov rcx, 1_000_000_000
loop $

mov rax, 1
syscall

jmp cycle

hello: db "Hello, World!"
.len EQU $ - hello
msg: db "I'm a Thread!"
.len EQU $ - msg
