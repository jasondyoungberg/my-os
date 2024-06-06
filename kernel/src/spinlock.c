#include "spinlock.h"

#include "registers.h"
#include <stdatomic.h>
#include <stdbool.h>

#define IF_DISABLED 1
#define IF_ENABLED 2

void spin_acquire(atomic_int *lock) {
    int val = read_rflags() & RFLAGS_IF ? IF_ENABLED : IF_DISABLED;

    __asm__ volatile("cli");

    while (1) {
        int expected = 0;
        if (atomic_compare_exchange_weak_explicit(lock, &expected, val,
                                                  memory_order_acquire,
                                                  memory_order_relaxed)) {
            break;
        }
        __builtin_ia32_pause(); // voluntary CPU yield
    }
}

void spin_release(atomic_int *lock) {
    int val = atomic_exchange(lock, 0);

    if (val == IF_ENABLED)
        __asm__ volatile("sti");
}
