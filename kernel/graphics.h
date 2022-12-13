#ifndef MYOS_KERNEL_GRAPHICS_H_
#define MYOS_KERNEL_GRAPHICS_H_

#include <stdint.h>

typedef uint32_t color_t;

extern int graphics_width;
extern int graphics_height;

/**
 * Initializes the graphics subsystem.
 * @return 0 on success, -1 on failure.
 */
int graphics_init(void);

/**
 * Refreshes the screen.
 * @return 0 on success, -1 on failure.
 */
int graphics_refresh(void);

/**
 * Converts an RGB color to a 32-bit color.
 * @param r The red component.
 * @param g The green component.
 * @param b The blue component.
 * @return The color.
 */
color_t color_rgb(uint8_t r, uint8_t g, uint8_t b);

/**
 * Draws a pixel.
 * @param x The x coordinate.
 * @param y The y coordinate.
 * @param color The color.
 */
void draw_pix(int x, int y, uint32_t color);

/**
 * Fills the screen with a color.
 * @param color The color.
 * @return 0 on success, -1 on failure.
 */
int draw_fill(uint32_t color);

#endif