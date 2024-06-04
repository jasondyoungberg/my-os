#ifndef ALLOC_H_
#define ALLOC_H_

#include <stddef.h>

void *kmalloc(size_t size);
void kfree(void *ptr);

#endif
