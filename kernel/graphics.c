#include "graphics.h"

#include <limine.h>
#include <stddef.h>
#include <stdint.h>

#include "serial.h"

// Request the framebuffer from limine
static volatile struct limine_framebuffer_request framebuffer_request = {
    .id = LIMINE_FRAMEBUFFER_REQUEST, .revision = 0};

static uint32_t* framebuffer;
static uint32_t backbuffer[10000000];  // TODO: Dynamically allocate this

static int width;
static int height;
static int pitch;

int graphics_width;
int graphics_height;

int graphics_init(void) {
    if (framebuffer_request.response == NULL) return -1;

    framebuffer = framebuffer_request.response->framebuffers[0]->address;
    width = framebuffer_request.response->framebuffers[0]->width;
    height = framebuffer_request.response->framebuffers[0]->height;
    pitch = framebuffer_request.response->framebuffers[0]->pitch;
    pitch = (pitch >> 2);  // Convert from bytes to 32-bit words

    graphics_width = width;
    graphics_height = height;

    return 0;
}

int graphics_refresh(void) {
    for (int i = 0; i < height * pitch; i++) {
        framebuffer[i] = backbuffer[i];
    }

    return 0;
}

color_t color_rgb(uint8_t r, uint8_t g, uint8_t b) {
    return (r << 16) | (g << 8) | b;
}

void draw_pix(int x, int y, color_t color) {
    backbuffer[y * pitch + x] = color;
}

int draw_fill(color_t color) {
    for (int y = 0; y < height; y++) {
        for (int x = 0; x < width; x++) {
            backbuffer[y * pitch + x] = color;
        }
    }

    return 0;
}