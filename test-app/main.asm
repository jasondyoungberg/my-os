[bits 64]
[org 0]

start:

mov al, 'A'
out 0xE9, al
out 0xE9, al
out 0xE9, al
int3
jmp start
