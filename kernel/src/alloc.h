#ifndef ALLOC_H
#define ALLOC_H

#include <stddef.h>

void *kmalloc(size_t size);
void kfree(void *ptr);

#endif
