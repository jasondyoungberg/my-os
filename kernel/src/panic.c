#include "panic.h"

#include "debugcon.h"

void panic(const char *msg) {
    __asm__("cli");

    kprint_str("Kernel panic: ");
    kprint_str(msg);
    kprintln();

    for (;;) {
        __asm__("hlt");
    }
}
