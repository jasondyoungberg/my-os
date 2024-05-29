[bits 64]
[org 0x1000]

main:

mov rax, [gpr_data]
mov rbx, [gpr_data + 8]
mov rcx, [gpr_data + 16]
mov rdx, [gpr_data + 24]
mov rsi, [gpr_data + 32]
mov rdi, [gpr_data + 40]
mov rbp, [gpr_data + 48]
mov r8, [gpr_data + 56]
mov r9, [gpr_data + 64]
mov r10, [gpr_data + 72]
mov r11, [gpr_data + 80]
mov r12, [gpr_data + 88]
mov r13, [gpr_data + 96]
mov r14, [gpr_data + 104]
mov r15, [gpr_data + 112]

movq mm0, [mmx_data]
movq mm1, [mmx_data + 8]
movq mm2, [mmx_data + 16]
movq mm3, [mmx_data + 24]
movq mm4, [mmx_data + 32]
movq mm5, [mmx_data + 40]
movq mm6, [mmx_data + 48]
movq mm7, [mmx_data + 56]

yield:
    push rax
    push rcx
    push r11
    mov rax, 24
    syscall
    pop r11
    pop rcx
    pop rax

gpr_test:
    add rax, rbx
    add rax, rcx
    add rax, rdx
    add rax, rsi
    add rax, rdi
    add rax, rbp
    add rax, r8
    add rax, r9
    add rax, r10
    add rax, r11
    add rax, r12
    add rax, r13
    add rax, r14
    add rax, r15

    cmp rax, 120
    jne .fail
    .pass:
        mov rax, 1
        mov rdi, 1
        mov rsi, msg_gpr_pass
        mov rdx, msg_gpr_pass.len
        syscall
        jmp .end
    .fail:
        mov rax, 1
        mov rdi, 1
        mov rsi, msg_gpr_fail
        mov rdx, msg_gpr_fail.len
        syscall
    .end:

mmx_test:
    paddq mm0, mm1
    paddq mm0, mm2
    paddq mm0, mm3
    paddq mm0, mm4
    paddq mm0, mm5
    paddq mm0, mm6
    paddq mm0, mm7

    movq [sum], mm0
    mov rax, [sum]
    cmp rax, 36
    jne .fail
    .pass:
        mov rax, 1
        mov rdi, 1
        mov rsi, msg_mmx_pass
        mov rdx, msg_mmx_pass.len
        syscall
        jmp .end
    .fail:
        mov rax, 1
        mov rdi, 2
        mov rsi, msg_mmx_fail
        mov rdx, msg_mmx_fail.len
        syscall
    .end:

jmp main

msg_gpr_pass: db `Genral Purpose Registers Test Passed!\n`
    .len EQU $ - msg_gpr_pass
msg_gpr_fail: db `Genral Purpose Registers Test Failed!\n`
    .len EQU $ - msg_gpr_fail

msg_mmx_pass: db `MMX Registers Test Passed!\n`
    .len EQU $ - msg_mmx_pass
msg_mmx_fail: db `MMX Registers Test Failed!\n`
    .len EQU $ - msg_mmx_fail

sum: dq 0

gpr_data: dq 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
mmx_data: dq 1, 2, 3, 4, 5, 6, 7, 8
