#include "memory/mapping.h"
#include "common/log.h"
#include "common/registers.h"
#include "memory/frame.h"
#include "requests.h"

void* hhdm(uint64_t phys) {
    return (void*)(hhdm_request.response->offset + phys);
}

typedef uint64_t entry_t;
typedef uint64_t flags_t;

uint64_t map_page(void* ptr, flags_t flags) {
    const uint64_t virt = (uint64_t)ptr;
    const int64_t l4_index = (virt >> 12 >> 9 >> 9 >> 9) & 0x1ff;
    const uint64_t l3_index = (virt >> 12 >> 9 >> 9) & 0x1ff;
    const uint64_t l2_index = (virt >> 12 >> 9) & 0x1ff;
    const uint64_t l1_index = (virt >> 12) & 0x1ff;

    entry_t* l4_table = hhdm(read_cr3() & 0x0ffffffffffff000);
    if ((l4_table[l4_index] & PAGEFLAG_P) == 0) {
        l4_table[l4_index] = frame_alloc() | flags | PAGEFLAG_P;
    } else if ((l4_table[l4_index] & 0x1ff) != (flags | PAGEFLAG_P)) {
        panic("todo: mismatched page table flags");
    }

    entry_t* l3_table = hhdm(l4_table[l4_index] & 0x0ffffffffffff000);
    if ((l3_table[l3_index] & PAGEFLAG_P) == 0) {
        l3_table[l3_index] = frame_alloc() | flags | PAGEFLAG_P;
    } else if ((l3_table[l3_index] & 0x1ff) != (flags | PAGEFLAG_P)) {
        panic("todo: mismatched page table flags");
    }

    entry_t* l2_table = hhdm(l3_table[l3_index] & 0x0ffffffffffff000);
    if ((l2_table[l2_index] & PAGEFLAG_P) == 0) {
        l2_table[l2_index] = frame_alloc() | flags | PAGEFLAG_P;
    } else if ((l2_table[l2_index] & 0x1ff) != (flags | PAGEFLAG_P)) {
        panic("todo: mismatched page table flags");
    }

    entry_t* l1_table = hhdm(l2_table[l2_index] & 0x0ffffffffffff000);
    if ((l1_table[l1_index] & PAGEFLAG_P) == 0) {
        l1_table[l1_index] = frame_alloc() | flags | PAGEFLAG_P;
    } else if ((l1_table[l1_index] & 0x1ff) != (flags | PAGEFLAG_P)) {
        panic("todo: mismatched page table flags");
    }

    return l1_table[l1_index] & 0x0ffffffffffff000;
}
