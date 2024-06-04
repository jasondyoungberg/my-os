#include "alloc.h"

#include <stdint.h>

#define HEAP_SIZE 256 * 1024 * 1024

static uint8_t heap[HEAP_SIZE];

static size_t heap_allocated = 0;

void *kmalloc(size_t size) {
    if (heap_allocated + size > HEAP_SIZE) {
        return NULL;
    }

    void *ptr = heap + heap_allocated;
    heap_allocated += size;
    heap_allocated = (heap_allocated + 7) & ~7;
    return ptr;
}

void kfree(void *ptr) {
    // Do nothing
}
