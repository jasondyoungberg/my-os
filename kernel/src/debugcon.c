#include "debugcon.h"

#include "io.h"
#include "spinlock.h"
#include <stdarg.h>
#include <stdatomic.h>

#define ERROR_MESSAGE "<err>"

static void kprint_signed(long val, int base);
static void kprint_unsigned(unsigned long val, int base);
static void kprint_ptr(const void *ptr);
static void kprint_char(char c);
static void kprint_str(const char *str);

static atomic_flag lock = ATOMIC_FLAG_INIT;

// https://cplusplus.com/reference/cstdio/printf/
void kprintf(const char *fmt, ...) {
    va_list args;
    va_start(args, fmt);

    acquire(&lock);

    while (*fmt) {
        if (*fmt == '%') {
            fmt++;
            switch (*fmt) {
            case 'd':
            case 'i':
                kprint_signed(va_arg(args, int), 10);
                break;
            case 'u':
                kprint_unsigned(va_arg(args, unsigned int), 10);
                break;
            case 'o':
                kprint_unsigned(va_arg(args, unsigned int), 8);
                break;
            case 'x':
            case 'X':
                kprint_unsigned(va_arg(args, unsigned int), 16);
                break;
            case 'c':
                kprint_char((char)va_arg(args, int));
                break;
            case 's':
                kprint_str(va_arg(args, const char *));
                break;
            case 'p':
                kprint_ptr(va_arg(args, const void *));
                break;

            case 'h':
                fmt++;
                switch (*fmt) {
                case 'd':
                case 'i':
                    kprint_signed((short)va_arg(args, int), 10);
                    break;
                case 'u':
                    kprint_unsigned((unsigned short)va_arg(args, unsigned int),
                                    10);
                    break;
                case 'o':
                    kprint_unsigned((unsigned short)va_arg(args, unsigned int),
                                    8);
                    break;
                case 'x':
                case 'X':
                    kprint_unsigned((unsigned short)va_arg(args, unsigned int),
                                    16);
                    break;
                default:
                    kprint_char('%');
                    kprint_char('h');
                    kprint_char(*fmt);
                    break;
                }
                break;

            case 'l':
                fmt++;
                switch (*fmt) {
                case 'd':
                case 'i':
                    kprint_signed(va_arg(args, long), 10);
                    break;
                case 'u':
                    kprint_unsigned(va_arg(args, unsigned long), 10);
                    break;
                case 'o':
                    kprint_unsigned(va_arg(args, unsigned long), 8);
                    break;
                case 'x':
                case 'X':
                    kprint_unsigned(va_arg(args, unsigned long), 16);
                    break;
                default:
                    kprint_char('%');
                    kprint_char('l');
                    kprint_char(*fmt);
                    break;
                }
                break;

            case '%':
                kprint_char('%');
                break;
            default:
                kprint_char('%');
                kprint_char(*fmt);
                break;
            }
        } else {
            kprint_char(*fmt);
        }
        fmt++;
    }

    release(&lock);

    va_end(args);
}

static void kprint_char(char c) { outb(0xe9, c); }

static void kprint_str(const char *str) {
    while (*str) {
        kprint_char(*str);
        str++;
    }
}

static void kprint_signed(long val, int base) {
    if (val < 0) {
        kprint_char('-');
        val = -val;
    }

    kprint_unsigned(val, base);
}

static void kprint_unsigned(unsigned long val, int base) {
    if (val == 0) {
        kprint_char('0');
        return;
    }

    if (base < 8 || base > 16) {
        kprint_str("error");
        return;
    }

    char buf[24];
    int i = 0;
    while (val > 0) {
        buf[i++] = "0123456789abcdef"[val % base];
        val /= base;
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
