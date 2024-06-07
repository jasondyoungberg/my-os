#include "memory/heap.h"
#include "common/log.h"
#include "common/memory.h"

#include <stdatomic.h>
#include <stdint.h>

#define HEAP_SIZE 256 * 1024 * 1024

static uint8_t g_heap[HEAP_SIZE];
static atomic_size_t g_allocated = 0;

void* kmalloc(size_t size) {
    size = (size + 15) & ~15;

    size_t addr = atomic_fetch_add(&g_allocated, size);

    if (addr > HEAP_SIZE)
        panic("kmalloc(%d) failed", size);

    log_trace("kmalloc(%d)", size);
    return g_heap + addr;
}

void* kmalloc_zero(size_t size) {
    void* ptr = kmalloc(size);
    memset(ptr, 0, size);
    return ptr;
}

void kfree(void* ptr) { (void)ptr; }
