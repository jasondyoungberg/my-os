#include "alloc.h"
#include "debugcon.h"
#include "display.h"
#include "io.h"
#include "mem_ops.h"
#include "panic.h"
#include "requests.h"
#include <stdbool.h>

// The following will be our kernel's entry point.
// If renaming _start() to something else, make sure to change the
// linker script accordingly.
void _start(void) {
    kprint_str("Hello, world!\n");

    // Ensure the bootloader actually understands our base revision (see spec).
    if (LIMINE_BASE_REVISION_SUPPORTED == false) {
        panic("Bootloader revision not supported");
    }

    // Ensure we got a framebuffer.
    if (framebuffer_request.response == NULL ||
        framebuffer_request.response->framebuffer_count < 1) {
        panic("No framebuffer available");
    }

    for (;;) {
        for (int t = 0; t < 256; t++) {
            for (int y = 0; y < display_height(); y++) {
                for (int x = 0; x < display_width(); x++) {
                    set_pixel(x, y, (struct Color){x, y, t});
                }
            }
        }
    }

    outb(0xe9, 0x21);

    for (;;) {
        asm("hlt");
    }
}
