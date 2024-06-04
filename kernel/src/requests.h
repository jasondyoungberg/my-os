#ifndef REQUESTS_H
#define REQUESTS_H
#include <limine.h>
#include <stdint.h>

extern volatile uint64_t limine_base_revision[3];
extern volatile struct limine_framebuffer_request framebuffer_request;

#endif
