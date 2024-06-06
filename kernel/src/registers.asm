section .text

global read_rflags
read_rflags:
    pushf
    pop rax
    ret

global write_rflags
write_rflags:
    push rdi
    popf
    ret

global read_cr0
read_cr0:
    mov rax, cr0
    ret

global read_cr2
read_cr2:
    mov rax, cr2
    ret

global read_cr3
read_cr3:
    mov rax, cr3
    ret

global read_cr4
read_cr4:
    mov rax, cr4
    ret
