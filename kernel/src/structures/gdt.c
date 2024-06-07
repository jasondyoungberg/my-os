#include "structures/gdt.h"

#include "common/log.h"
#include "memory/heap.h"
#include "memory/mapping.h"
#include <stdatomic.h>
#include <stdint.h>

#define GDT_KERNEL_CODE 0x08
#define GDT_KERNEL_DATA 0x10
#define GDT_USER_DATA 0x18
#define GDT_USER_CODE 0x20

#define TSS_STACK_SIZE (64 * 1024)

typedef uint64_t gdt_entry_t;
typedef struct __attribute__((packed)) {
    uint16_t limit;
    gdt_entry_t* base;
} gdtr_t;

typedef struct __attribute__((packed)) {
    uint32_t _1;
    void* rsp0;
    void* rsp1;
    void* rsp2;
    uint64_t _2;
    void* ist1;
    void* ist2;
    void* ist3;
    void* ist4;
    void* ist5;
    void* ist6;
    void* ist7;
    uint64_t _3;
    uint16_t _4;
    uint16_t iopb;
} tss_t;

static void* create_stack();

extern void gdt_load(gdtr_t*);
void gdt_init() {
    int entries = 8;
    gdt_entry_t* gdt = (gdt_entry_t*)kmalloc(entries * sizeof(gdt_entry_t));
    gdtr_t* gdtr = (gdtr_t*)kmalloc(sizeof(gdtr_t));
    tss_t* tss = (tss_t*)kmalloc(sizeof(tss_t));

    tss->rsp0 = create_stack();
    tss->ist1 = create_stack();
    tss->ist2 = create_stack();
    tss->ist3 = create_stack();
    tss->ist4 = create_stack();
    tss->ist5 = create_stack();
    tss->ist6 = create_stack();
    tss->ist7 = create_stack();

    uint64_t tss_limit = sizeof(tss_t) - 1;
    uint64_t tss_base1 = ((uint64_t)tss & 0x00ffffff) << 16;
    uint64_t tss_flags = (uint64_t)0x89 << 40;
    uint64_t tss_base2 = ((uint64_t)tss & 0xff000000) << 32;

    gdt[0] = 0;
    gdt[1] = 0x00209A0000000000; // Kernel Code
    gdt[2] = 0x0020920000000000; // Kernel Data
    gdt[3] = 0x0020F20000000000; // User Data
    gdt[4] = 0x0020FA0000000000; // User Code
    gdt[5] = tss_limit | tss_base1 | tss_flags | tss_base2;
    gdt[6] = (uint64_t)tss >> 32;
    gdt[7] = 0;

    gdtr->limit = entries * sizeof(uint64_t) - 1;
    gdtr->base = gdt;

    gdt_load(gdtr);
}

static atomic_ulong g_stack_next = 0xffff900000000000;

static void* create_stack() {
    uint64_t start = atomic_fetch_add(&g_stack_next, TSS_STACK_SIZE);
    uint64_t end = start + TSS_STACK_SIZE;

    for (uint64_t i = 0; i < TSS_STACK_SIZE; i += 4096) {
        map_page((void*)(start + i), PAGEFLAG_RW);
    }

    return (void*)end;
}
