#include <limine.h>
#include <stddef.h>
#include <stdint.h>

#include "serial.h"

// Request the terminal from limine
static volatile struct limine_terminal_request terminal_request = {
    .id = LIMINE_TERMINAL_REQUEST, .revision = 0};

// Halt the CPU
static void halt(void) {
    for (;;) {
        asm("hlt");
    }
}

void _start(void) {
    // Ensure we got a terminal
    if (terminal_request.response == NULL ||
        terminal_request.response->terminal_count < 1)
        halt();

    // Print to the terminal
    struct limine_terminal *terminal = terminal_request.response->terminals[0];
    terminal_request.response->write(terminal, "Hello, World! I'm a console!",
                                     28);

    halt();
}