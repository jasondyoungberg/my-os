#include "common/log.h"
#include "common/console.h"
#include "common/spinlock.h"

static atomic_ulong g_lock = 0;

void log_trace(const char* fmt, ...) {
    va_list args;
    va_start(args, fmt);
    log(LogTrace, fmt, args);
    va_end(args);
}

void log_debug(const char* fmt, ...) {
    va_list args;
    va_start(args, fmt);
    log(LogDebug, fmt, args);
    va_end(args);
}

void log_info(const char* fmt, ...) {
    va_list args;
    va_start(args, fmt);
    log(LogInfo, fmt, args);
    va_end(args);
}

void log_warn(const char* fmt, ...) {
    va_list args;
    va_start(args, fmt);
    log(LogWarn, fmt, args);
    va_end(args);
}

void log_error(const char* fmt, ...) {
    va_list args;
    va_start(args, fmt);
    log(LogError, fmt, args);
    va_end(args);
}

void panic(const char* fmt, ...) {
    __asm__("cli");
    va_list args;
    va_start(args, fmt);
    log(LogPanic, fmt, args);
    va_end(args);
    for (;;)
        __asm__("hlt");
}

void log(log_level_t level, const char* fmt, va_list args) {
    spin_acquire(&g_lock);
    switch (level) {
    case LogTrace:
        kprintf("\x1b[90m[TRACE] ");
        break;
    case LogDebug:
        kprintf("\x1b[92m[DEBUG] ");
        break;
    case LogInfo:
        kprintf("\x1b[96m[INFO] ");
        break;
    case LogWarn:
        kprintf("\x1b[93m[WARN] ");
        break;
    case LogError:
        kprintf("\x1b[91m[ERROR] ");
        break;
    case LogPanic:
        kprintf("\x1b[97;41m[PANIC]\x1b[91;49m ");
        break;
    default:
        break;
    }

    vkprintf(fmt, args);
    kprintf("\x1b[0m\n");

    spin_release(&g_lock);
}
