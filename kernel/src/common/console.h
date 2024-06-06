#pragma once
#include <stdarg.h>
#include <stdint.h>

void kprintf(const char* fmt, ...);
void vkprintf(const char* fmt, va_list args);
