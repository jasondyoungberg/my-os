#include "serial.h"

#include <stdint.h>

static inline void outb(uint16_t port, uint8_t val) {
    asm volatile("outb %0, %1" : : "a"(val), "Nd"(port));
}

static inline uint8_t inb(uint16_t port) {
    uint8_t ret;
    asm volatile("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

int serial_init(uint16_t port, int baud) {
    uint16_t divisor = 115200 / baud;

    outb(port + 1, 0x00);            // Disable all interrupts
    outb(port + 3, 0x80);            // Enable DLAB (set baud rate divisor)
    outb(port + 0, divisor & 0xFF);  // Set divisor (lo byte)
    outb(port + 1, divisor >> 8);    //             (hi byte)
    outb(port + 3, 0x03);            // 8 bits, no parity, one stop bit
    outb(port + 2, 0xC7);  // Enable FIFO, clear them, with 14-byte threshold
    outb(port + 4, 0x0B);  // IRQs enabled, RTS/DSR set
    outb(port + 4, 0x1E);  // Set in loopback mode, test the serial chip
    outb(port + 0, 0xAE);  // Test serial chip (send byte 0xAE and check if
                           // serial returns same byte)

    if (inb(port + 0) != 0xAE) return -1;

    outb(port + 4, 0x0F);  // Set normal operation mode
    return 0;
}

int serial_send(uint16_t port, char data) {
    while ((inb(port + 5) & 0x20) == 0) continue;
    outb(port, data);
    return 0;
}

int serial_print(uint16_t port, char* str) {
    for (int i = 0; str[i] != 0; i++) serial_send(port, str[i]);
    return 0;
}

int serial_println(uint16_t port, char* str) {
    serial_print(port, str);
    serial_send(port, '\r');
    serial_send(port, '\n');
    return 0;
}