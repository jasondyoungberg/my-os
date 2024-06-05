#include "rflags.h"

uint64_t read_rflags() {
    uint64_t rflags;
    __asm__ volatile("pushfq; popq %0" : "=r"(rflags));
    return rflags;
}

void write_rflags(uint64_t rflags) {
    __asm__ volatile("pushq %0; popfq" : : "r"(rflags));
}
