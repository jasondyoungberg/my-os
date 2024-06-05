#ifndef REQUESTS_H_
#define REQUESTS_H_

#include <stdatomic.h>

void acquire(atomic_flag *);
void release(atomic_flag *);

#endif
