#include "panic.h"

#include "debugcon.h"

void panic(const char *msg) {
    __asm__("cli");

    kprintf("Kernel panic:\n%s\n", msg);

    for (;;)
        __asm__("hlt");
}
