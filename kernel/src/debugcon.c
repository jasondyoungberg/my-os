#include "debugcon.h"

#include "io.h"
#include <stdarg.h>

void kprintf(const char *fmt, ...) {
    va_list args;
    va_start(args, fmt);

    while (*fmt) {
        if (*fmt == '%') {
            fmt++;
            switch (*fmt) {
            case 'd': // Decimal
                kprint_dec(va_arg(args, long));
                break;
            case 'c': // Char
                kprint_char(va_arg(args, int));
                break;
            case 's': // String
                kprint_str(va_arg(args, const char *));
                break;
            case 'x': // Hexadecimal
                kprint_hex(va_arg(args, unsigned long));
                break;
            case 'p': // Pointer
                kprint_ptr(va_arg(args, const void *));
                break;
            case '%':
                kprint_char('%');
                break;
            default:
                kprint_char('?');
                break;
            }
        } else {
            kprint_char(*fmt);
        }
        fmt++;
    }

    va_end(args);
}

void kprint_char(char c) { outb(0xe9, c); }

void kprint_str(const char *str) {
    while (*str) {
        kprint_char(*str);
        str++;
    }
}

void kprint_dec(long val) {
    if (val < 0) {
        kprint_char('-');
        val = -val;
    }

    if (val == 0) {
        kprint_char('0');
        return;
    }

    char buf[19];
    int i = 0;
    while (val > 0) {
        buf[i++] = '0' + (val % 10);
        val /= 10;
    }

    for (int j = i - 1; j >= 0; j--) {
        kprint_char(buf[j]);
    }
}

void kprint_hex(unsigned long val) {
    kprint_str("0x");

    if (val == 0) {
        kprint_char('0');
        return;
    }

    char buf[16];
    int i = 0;
    while (val > 0) {
        buf[i++] = "0123456789abcdef"[val % 16];
        val /= 16;
    }

    for (int j = i - 1; j >= 0; j--) {
        kprint_char(buf[j]);
    }
}

void kprint_ptr(const void *ptr) {
    kprint_str("0x");
    uint64_t val = (uint64_t)ptr;
    for (int i = 44; i >= 0; i -= 4) {
        kprint_char("0123456789abcdef"[(val >> i) & 0xf]);
    }
}
