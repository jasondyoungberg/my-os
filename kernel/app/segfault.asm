[bits 64]
[org 0x1000]

cycle:
mov rax, [0xdeadbeef]
jmp cycle
