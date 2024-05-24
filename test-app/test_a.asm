[bits 64]
[org 0x1000]

mov al, 'A'

loop:

out 0xE9, al
out 0xE9, al
out 0xE9, al
int3

jmp loop
