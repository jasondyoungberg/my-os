#ifndef PANIC_H_
#define PANIC_H_

#include <stdnoreturn.h>

noreturn void panic(const char *fmt, ...);

#endif
