[bits 64]
[org 0x1000]

mov rax, 2
mov rdi, hello
mov rsi, hello.len
syscall

main:
    mov rcx, 200_000_000
    loop $
    mov rax, 2
    mov rdi, msg
    mov rsi, msg.len
    syscall
    jmp main


hello: db `Hello from userland, again!\n`
    .len EQU $ - hello

msg: db `Thread B\n`
    .len EQU $ - msg
