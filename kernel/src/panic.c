#include "panic.h"

#include "debugcon.h"

void panic(const char *msg) {
    asm("cli");

    kprint_str("Kernel panic: ");
    kprint_str(msg);
    kprintln();

    for (;;) {
        asm("hlt");
    }
}
