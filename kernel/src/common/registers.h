#pragma once

#include <stdint.h>

#define RFLAGS_CF (1 << 0)      // Carry Flag
#define RFLAGS_PF (1 << 2)      // Parity Flag
#define RFLAGS_AF (1 << 4)      // Auxiliary Carry Flag
#define RFLAGS_ZF (1 << 6)      // Zero Flag
#define RFLAGS_SF (1 << 7)      // Sign Flag
#define RFLAGS_TF (1 << 8)      // Trap Flag
#define RFLAGS_IF (1 << 9)      // Interrupt Enable Flag
#define RFLAGS_DF (1 << 10)     // Direction Flag
#define RFLAGS_OF (1 << 11)     // Overflow Flag
#define RFLAGS_IOPL_0 (1 << 12) // I/O Privilege Level (bit 0)
#define RFLAGS_IOPL_1 (1 << 13) // I/O Privilege Level (bit 1)
#define RFLAGS_NT (1 << 14)     // Nested Task
#define RFLAGS_RF (1 << 16)     // Resume Flag
#define RFLAGS_VM (1 << 17)     // Virtual 8086 Mode
#define RFLAGS_AC (1 << 18)     // Alignment Check
#define RFLAGS_VIF (1 << 19)    // Virtual Interrupt Flag
#define RFLAGS_VIP (1 << 20)    // Virtual Interrupt Pending
#define RFLAGS_ID (1 << 21)     // ID Flag

uint64_t read_rflags();
uint64_t read_cr0();
uint64_t read_cr2();
uint64_t read_cr3();

void write_rflags(uint64_t);
