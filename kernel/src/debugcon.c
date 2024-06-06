#include "debugcon.h"

#include "io.h"
#include "spinlock.h"
#include <stdatomic.h>

static void kprint_signed(long val, int padding);
static void kprint_unsigned(unsigned long val, int padding);
static void kprint_hex(unsigned long val, int padding);
static void kprint_base(unsigned long val, int padding, int base);

static void kprint_ptr(const void *ptr);
static void kprint_char(char c);
static void kprint_str(const char *str);

static atomic_int lock = 0;

static enum state { StateNormal, StatePadding, StateSpecifier };

void kprintf(const char *fmt, ...) {
    va_list args;
    va_start(args, fmt);
    vkprintf(fmt, args);
    va_end(args);
}

void vkprintf(const char *fmt, va_list args) {
    acquire(&lock);

    enum state state = StateNormal;
    int padding = -1;

    while (*fmt) {
        switch (state) {

        case StateNormal:
            switch (*fmt) {
            case '%':
                fmt++;
                if (*fmt == '%') {
                    kprint_char('%');
                } else {
                    state = StatePadding;
                }
                break;
            default:
                kprint_char(*fmt);
                fmt++;
                break;
            }
            break;

        case StatePadding:
            switch (*fmt) {
            case '0':
            case '1':
            case '2':
            case '3':
            case '4':
            case '5':
            case '6':
            case '7':
            case '8':
            case '9':
                if (padding == -1)
                    padding = 0;

                padding *= 10;
                padding += (*fmt) - '0';
                fmt++;
                break;
            default:
                if (padding == -1)
                    padding = 1;
                state = StateSpecifier;
                break;
            }
            break;

        case StateSpecifier:
            switch (*fmt) {
            case 'c':
                kprint_char(*fmt);
                break;
            case 's':
                kprint_str(va_arg(args, const char *));
                break;
            case 'd':
            case 'i':
                kprint_signed(va_arg(args, long), padding);
                break;
            case 'x':
            case 'X':
                kprint_hex(va_arg(args, unsigned long), padding);
                break;
            case 'u':
                kprint_unsigned(va_arg(args, unsigned long), padding);
                break;
            case 'p':
                kprint_ptr(va_arg(args, const void *));
                break;
            default:
                kprint_str("<fmt error>");
                break;
            }
            state = StateNormal;
            padding = 0;
            fmt++;
            break;

        default:
            kprint_str("<internal error>");
            fmt++;
            break;
        }
    }

    release(&lock);
}

static void kprint_char(char c) { outb(0xe9, c); }

static void kprint_str(const char *str) {
    while (*str) {
        kprint_char(*str);
        str++;
    }
}

static void kprint_signed(long val, int padding) {
    if (val < 0) {
        kprint_char('-');
        val = -val;
    }

    kprint_unsigned(val, padding);
}

static void kprint_unsigned(unsigned long val, int padding) {
    kprint_base(val, padding, 10);
}
static void kprint_hex(unsigned long val, int padding) {
    kprint_base(val, padding, 16);
}

static void kprint_base(unsigned long val, int padding, int base) {
    if (base < 10 || base > 16 || padding < 0) {
        kprint_str("<internal error>");
        return;
    }

    char buf[32];
    int i = 0;
    while (val > 0) {
        buf[i] = "0123456789abcdef"[val % base];
        val /= base;
        i += 1;
    }

    while (i < padding) {
        kprint_char('0');
        padding--;
    }

    for (int j = i - 1; j >= 0; j--) {
        kprint_char(buf[j]);
    }
}

static void kprint_ptr(const void *ptr) {
    uint64_t val = (uint64_t)ptr;
    for (int i = 44; i >= 0; i -= 4) {
        kprint_char("0123456789abcdef"[(val >> i) & 0xf]);
    }
}
