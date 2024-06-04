#ifndef DISPLAY_H_
#define DISPLAY_H_
#include <stdint.h>

struct Color {
    uint8_t r;
    uint8_t g;
    uint8_t b;
};

void set_pixel(uint32_t x, uint32_t y, struct Color color);
int display_width();
int display_height();

#endif
