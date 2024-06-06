#include "structures/gdt.h"

#include "common/panic.h"
#include "memory/heap.h"
#include <stdint.h>
#include <stdlib.h>

#define GDT_KERNEL_CODE 0x08
#define GDT_KERNEL_DATA 0x10
#define GDT_USER_DATA 0x18
#define GDT_USER_CODE 0x20

extern void gdt_load(uint16_t, uint64_t);
void gdt_init() {
    int entries = 5;
    uint64_t* gdt = (uint64_t*)kmalloc(entries * sizeof(uint64_t));
    if (gdt == NULL)
        panic("Failed to allocate memory for GDT");

    gdt[0] = 0;
    gdt[1] = 0x00209A0000000000;
    gdt[2] = 0x0020920000000000;
    gdt[3] = 0x0020F20000000000;
    gdt[4] = 0x0020FA0000000000;

    uint16_t limit = entries * sizeof(uint64_t) - 1;
    uint64_t base = (uint64_t)gdt;

    gdt_load(limit, base);
}
