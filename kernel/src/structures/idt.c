#include "idt.h"

#include "common/console.h"
#include "common/log.h"
#include "common/registers.h"
#include <stdint.h>

typedef struct {
    uint16_t isr_low;   // The lower 16 bits of the ISR's address
    uint16_t kernel_cs; // The GDT segment selector that the CPU will load into
                        // CS before calling the ISR
    uint8_t ist; // The IST in the TSS that the CPU will load into RSP; set to
                 // zero for now
    uint8_t attributes; // Type and attributes; see the IDT page
    uint16_t
        isr_mid; // The higher 16 bits of the lower 32 bits of the ISR's address
    uint32_t isr_high; // The higher 32 bits of the ISR's address
    uint32_t reserved; // Set to zero
} __attribute__((packed)) idt_entry_t;

__attribute__((aligned(0x10))) static idt_entry_t
    idt[256]; // Create an array of IDT entries; aligned for performance

typedef struct {
    uint16_t limit;
    uint64_t base;
} __attribute__((packed)) idtr_t;

static idtr_t idtr;

extern void* isr_stub_table[];

void idt_set_descriptor(uint8_t vector, void* isr, uint8_t flags) {
    idt_entry_t* descriptor = &idt[vector];

    descriptor->isr_low = (uint64_t)isr & 0xFFFF;
    descriptor->kernel_cs = 0x08;
    descriptor->ist = 0;
    descriptor->attributes = flags;
    descriptor->isr_mid = ((uint64_t)isr >> 16) & 0xFFFF;
    descriptor->isr_high = ((uint64_t)isr >> 32) & 0xFFFFFFFF;
    descriptor->reserved = 0;
}

void idt_init() {
    idtr.base = (uintptr_t)&idt[0];
    idtr.limit = (uint16_t)sizeof(idt_entry_t) * 256 - 1;

    for (int vector = 0; vector < 256; vector++) {
        idt_set_descriptor(vector, isr_stub_table[vector], 0x8E);
    }

    __asm__ volatile("lidt %0" : : "m"(idtr));
}

typedef struct {
    uint64_t rip;
    uint16_t cs;
    uint64_t rflags;
    uint64_t rsp;
    uint16_t ss;
} stackFrame_t;

typedef struct {
    uint64_t rax;
    uint64_t rbx;
    uint64_t rcx;
    uint64_t rdx;
    uint64_t rsi;
    uint64_t rdi;
    uint64_t rbp;
    uint64_t r8;
    uint64_t r9;
    uint64_t r10;
    uint64_t r11;
    uint64_t r12;
    uint64_t r13;
    uint64_t r14;
    uint64_t r15;
} registers_t;

void exception_handler(int vector, stackFrame_t* stack_frame, uint64_t err_code,
                       registers_t* regs) {
    switch (vector) {
    case 0:
        log_warn("Divide Error\n\tRIP: %p", stack_frame->rip);
        break;
    case 1:
        log_info("Debug\n\tRIP: %p", stack_frame->rip);
        break;
    case 2:
        panic("NMI Interrupt\n\tRIP: %p", stack_frame->rip);
        break;
    case 3:
        log_info("Breakpoint\n\tRIP: %p", stack_frame->rip);
        break;
    case 4:
        log_warn("Overflow\n\tRIP: %p", stack_frame->rip);
        break;
    case 5:
        panic("BOUND Range Exceeded\n\tRIP: %p", stack_frame->rip);
        break;
    case 6:
        panic("Invalid Opcode\n\tRIP: %p", stack_frame->rip);
        break;
    case 7:
        panic("Device Not Available\n\tRIP: %p", stack_frame->rip);
        break;
    case 8:
        panic("Double Fault\n\tRIP: %p", stack_frame->rip);
        break;
    case 10:
        panic("Invalid TSS (%d)\n\tRIP: %p\n", err_code, stack_frame->rip);
        break;
    case 11:
        panic("Segment Not Present (%d)\n\tRIP: %p\n", err_code,
              stack_frame->rip);
        break;
    case 12:
        panic("Stack-Segment Fault (%d)\n\tRIP: %p\n", err_code,
              stack_frame->rip);
        break;
    case 13:
        panic("General Protection (%d)\n\tRIP: %p\n", err_code,
              stack_frame->rip);
        break;
    case 14:
        panic("Page fault (%d)\n\tRIP: %p\n\tCR2: %p\n", err_code,
              stack_frame->rip, read_cr2());
        break;
    case 16:
        panic("Math Fault\n\tRIP: %p", stack_frame->rip);
        break;
    case 17:
        panic("Alignment Check\n\tRIP: %p", stack_frame->rip);
        break;
    case 18:
        panic("Machine Check\n\tRIP: %p", stack_frame->rip);
        break;
    case 19:
        panic("SIMD Floating-Point Exception\n\tRIP: %p", stack_frame->rip);
        break;
    case 20:
        panic("Virtualization Exception\n\tRIP: %p", stack_frame->rip);
        break;
    case 21:
        panic("Control Protection Exception (%d)\n\tRIP: %p\n", err_code,
              stack_frame->rip);
        break;
    default:
        kprintf("\nINT %d\n", vector);
        break;
    }
}
