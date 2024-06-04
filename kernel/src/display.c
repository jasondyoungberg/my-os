#include "display.h"
#include "requests.h"

void set_pixel(uint32_t x, uint32_t y, struct Color color) {
    struct limine_framebuffer *framebuffer =
        framebuffer_request.response->framebuffers[0];

    volatile uint32_t *fb_ptr = framebuffer->address;

    fb_ptr[y * (framebuffer->pitch / 4) + x] =
        (color.r << 16) | (color.g << 8) | color.b;
}

uint32_t display_width() {
    struct limine_framebuffer *framebuffer =
        framebuffer_request.response->framebuffers[0];

    return framebuffer->width;
}

uint32_t display_height() {
    struct limine_framebuffer *framebuffer =
        framebuffer_request.response->framebuffers[0];

    return framebuffer->height;
}
