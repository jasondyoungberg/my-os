[bits 64]
[org 0x1000]

mov rax, 1000
mov rdi, 1001
mov rsi, 1002
mov rdx, 1003
mov r10, 1004
mov r8, 1005
mov r9, 1006
syscall

jmp $
