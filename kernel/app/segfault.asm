[bits 64]
[org 0x1000]

cycle:
mov rbx, 0xdeadbeef
mov rax, [rax]
jmp cycle
