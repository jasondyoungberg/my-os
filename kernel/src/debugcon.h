#ifndef DEBUGCON_H_
#define DEBUGCON_H_

#include <stdint.h>

void kprintf(const char *fmt, ...);

void kprint_char(char c);
void kprint_str(const char *str);
void kprint_dec(long val);
void kprint_hex(unsigned long val);
void kprint_ptr(const void *ptr);

#endif
