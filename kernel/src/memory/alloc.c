#include "alloc.h"

#include <stdatomic.h>
#include <stdint.h>

#define HEAP_SIZE 256 * 1024 * 1024

static uint8_t heap[HEAP_SIZE];
static atomic_size_t heap_allocated = 0;

void *kmalloc(size_t size) {
    size = (size + 15) & ~15;

    size_t addr = atomic_fetch_add(&heap_allocated, size);

    if (addr > HEAP_SIZE) {
        return NULL;
    }

    return heap + addr;
}

void kfree(void *ptr) {
    // Do nothing
}
