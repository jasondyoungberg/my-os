#ifndef DEBUGCON_H
#define DEBUGCON_H

#include <stdint.h>

void kprint_char(char c);
void kprint_str(const char *str);
void kprintln(void);

void kprint_hex8(uint8_t val);
void kprint_hex16(uint16_t val);
void kprint_hex32(uint32_t val);
void kprint_hex64(uint64_t val);

void kprint_ptr(const void *ptr);

#endif
