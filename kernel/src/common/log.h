#pragma once
#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>

typedef enum {
    LogTrace,
    LogDebug,
    LogInfo,
    LogWarn,
    LogError,
    LogPanic
} log_level_t;

void log_trace(const char* fmt, ...);
void log_debug(const char* fmt, ...);
void log_info(const char* fmt, ...);
void log_warn(const char* fmt, ...);
void log_error(const char* fmt, ...);
__attribute__((noreturn)) void panic(const char* fmt, ...);

void log(log_level_t level, const char* fmt, va_list args);
