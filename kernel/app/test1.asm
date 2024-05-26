[bits 64]
[org 0x1000]

start:
mov al, `!`
out 0xe9, al
out 0xe9, al
out 0xe9, al
hlt
jmp start


