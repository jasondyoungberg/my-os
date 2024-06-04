#include "debugcon.h"

#include "io.h"

const char *HEX_CHARS = "0123456789abcdef";

void kprint_char(char c) { outb(0xe9, c); }

void kprint_str(const char *str) {
    while (*str) {
        kprint_char(*str);
        str++;
    }
}

void kprintln(void) { kprint_char('\n'); }

void kprint_hex8(uint8_t val) {
    kprint_str("0x");
    for (int i = 4; i >= 0; i -= 4) {
        kprint_char(HEX_CHARS[(val >> i) & 0xf]);
    }
}

void kprint_hex16(uint16_t val) {
    kprint_str("0x");
    for (int i = 12; i >= 0; i -= 4) {
        kprint_char(HEX_CHARS[(val >> i) & 0xf]);
    }
}

void kprint_hex32(uint32_t val) {
    kprint_str("0x");
    for (int i = 28; i >= 0; i -= 4) {
        kprint_char(HEX_CHARS[(val >> i) & 0xf]);

        if (i % 16 == 0 && i > 0)
            kprint_char('_');
    }
}

void kprint_hex64(uint64_t val) {
    kprint_str("0x");
    for (int i = 60; i >= 0; i -= 4) {
        kprint_char(HEX_CHARS[(val >> i) & 0xf]);

        if (i % 16 == 0 && i > 0)
            kprint_char('_');
    }
}

void kprint_ptr(const void *ptr) {
    kprint_str("0x");
    uint64_t val = (uint64_t)ptr;
    for (int i = 44; i >= 0; i -= 4) {
        kprint_char(HEX_CHARS[(val >> i) & 0xf]);

        if (i % 16 == 0 && i > 0)
            kprint_char('_');
    }
}
