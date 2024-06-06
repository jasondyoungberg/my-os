#include "panic.h"

#include "debugcon.h"

void panic(const char *fmt, ...) {
    __asm__("cli");

    va_list args;
    va_start(args, fmt);
    vkprintf(fmt, args);
    va_end(args);

    for (;;)
        __asm__("hlt");
}
