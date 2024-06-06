#ifndef MEMORY_ALLOC_H_
#define MEMORY_ALLOC_H_

#include <stddef.h>

void *kmalloc(size_t size);
void kfree(void *ptr);

#endif
