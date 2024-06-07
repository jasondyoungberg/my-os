#pragma once
#include <stdint.h>

void init_frame_alloc();
uint64_t frame_alloc();
uint64_t frame_alloc_zero();
void frame_free(uint64_t frame);
