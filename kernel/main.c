#include <limine.h>
#include <stddef.h>
#include <stdint.h>

#include "serial.h"

// Halt the CPU
static void halt(void) {
    for (;;) {
        asm("hlt");
    }
}

void _start(void) {
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