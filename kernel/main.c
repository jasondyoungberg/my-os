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
    // Initialize serial ports
    if (serial_init(SERIAL_COM1, 115200)) halt();
    if (serial_init(SERIAL_COM2, 115200)) halt();
    if (serial_init(SERIAL_COM3, 115200)) halt();
    if (serial_init(SERIAL_COM4, 115200)) halt();

    // Print to serial ports
    serial_println(SERIAL_COM1, "Hello, world! I'm COM1!");
    serial_println(SERIAL_COM2, "Hello, world! I'm COM2!");
    serial_println(SERIAL_COM3, "Hello, world! I'm COM3!");
    serial_println(SERIAL_COM4, "Hello, world! I'm COM4!");

    halt();
}