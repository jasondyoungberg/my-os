#include "debugcon.h"

#include "io.h"
#include "spinlock.h"
#include <stdarg.h>
#include <stdatomic.h>

static void kprint_signed(long val, int padding);
static void kprint_unsigned(unsigned long val, int padding);
static void kprint_hex(unsigned long val, int padding);
static void kprint_oct(unsigned long val, int padding);
static void kprint_base(unsigned long val, int padding, int base);

static void kprint_ptr(const void *ptr);
static void kprint_char(char c);
static void kprint_str(const char *str);

static atomic_int lock = 0;

static enum state {
    StateNormal,
    StateFlags,
    StateWidth,
    StatePrecision,
    StateLenth,
    StateSpecifier
};
static enum length { LengthHalf, LengthNormal, LengthLong };

// https://cplusplus.com/reference/cstdio/printf/
void kprintf(const char *fmt, ...) {
    va_list args;
    va_start(args, fmt);

    acquire(&lock);

    enum state state = StateNormal;
    int width = 0;
    enum length length = LengthNormal;

    while (*fmt) {
        switch (state) {

        case StateNormal:
            switch (*fmt) {
            case '%':
                state = StateFlags;
                fmt++;
                break;
            default:
                kprint_char(*fmt);
                fmt++;
                break;
            }
            break;

        case StateFlags:
            switch (*fmt) {
            case '-':
            case '+':
            case ' ':
            case '#':
            case '0':
                kprint_str("<todo>");
                fmt++;
                break;
            default:
                state = StateWidth;
                break;
            }
            break;

        case StateWidth:
            switch (*fmt) {
            case '0' ... '9':
                width *= 10;
                width += (*fmt) - '0';
                fmt++;
                break;
            case '*':
                kprint_str("<todo>");
                fmt++;
                break;
            case '.':
                state = StatePrecision;
                fmt++;
                break;
            default:
                state = StateLenth;
                break;
            }
            break;

        case StatePrecision:
            switch (*fmt) {
            case '0' ... '9':
            case '*':
                kprint_str("<todo>");
                fmt++;
                break;
            default:
                state = StateLenth;
                break;
            }
            break;

        case StateLenth:
            switch (*fmt) {
            case 'h':
                length = LengthHalf;
                fmt++;
                break;
            case 'l':
                length = LengthLong;
                fmt++;
                break;
            default:
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
                switch (length) {
                case LengthHalf:
                    kprint_signed((short)va_arg(args, int), width);
                    break;
                case LengthNormal:
                    kprint_signed(va_arg(args, int), width);
                    break;
                case LengthLong:
                    kprint_signed(va_arg(args, long), width);
                    break;
                default:
                    kprint_str("<err>");
                    break;
                }
                break;
            case 'o':
                switch (length) {
                case LengthHalf:
                    kprint_oct((unsigned short)va_arg(args, unsigned int),
                               width);
                    break;
                case LengthNormal:
                    kprint_oct(va_arg(args, unsigned int), width);
                    break;
                case LengthLong:
                    kprint_oct(va_arg(args, unsigned long), width);
                    break;
                default:
                    kprint_str("<err>");
                    break;
                }
                break;
            case 'x':
            case 'X':
                switch (length) {
                case LengthHalf:
                    kprint_hex((unsigned short)va_arg(args, unsigned int),
                               width);
                    break;
                case LengthNormal:
                    kprint_hex(va_arg(args, unsigned int), width);
                    break;
                case LengthLong:
                    kprint_hex(va_arg(args, unsigned long), width);
                    break;
                default:
                    kprint_str("<err>");
                    break;
                }
                break;
            case 'u':
                switch (length) {
                case LengthHalf:
                    kprint_unsigned((unsigned short)va_arg(args, unsigned int),
                                    width);
                    break;
                case LengthNormal:
                    kprint_unsigned(va_arg(args, unsigned int), width);
                    break;
                case LengthLong:
                    kprint_unsigned(va_arg(args, unsigned long), width);
                    break;
                default:
                    kprint_str("<err>");
                    break;
                }
                break;
            case 'n':
                kprint_str("<todo>");
                break;
            case 'p':
                kprint_ptr(va_arg(args, const void *));
                break;
            }
            state = StateNormal;
            width = 0;
            length = LengthNormal;
            fmt++;
            break;

        default:
            kprint_str("<err>");
            fmt++;
            break;
        }
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
static void kprint_oct(unsigned long val, int padding) {
    kprint_base(val, padding, 8);
}

static void kprint_base(unsigned long val, int padding, int base) {
    if (base < 8 || base > 16 || padding > 32) {
        kprint_str("<err>");
        return;
    }

    if (padding < 1)
        padding++;

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
