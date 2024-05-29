[bits 64]
[org 0x1000]

cycle:
mov rax, 1
mov rbx, 2
mov rcx, 3
mov rdx, 4
mov rsi, 5
mov rdi, 6
mov rbp, 7
mov r8, 8
mov r9, 9
mov r10, 10
mov r11, 11
mov r12, 12
mov r13, 13
mov r14, 14
mov r15, 15

mov rax, [0x12345678]

jmp cycle
