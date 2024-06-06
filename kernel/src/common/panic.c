#include "common/panic.h"

#include "common/console.h"

noreturn void panic(const char* fmt, ...) {
    __asm__("cli");

    va_list args;
    va_start(args, fmt);
    vkprintf(fmt, args);
    va_end(args);

    for (;;)
        __asm__("hlt");
}
