#ifndef MYOS_KERNEL_SERIAL_H
#define MYOS_KERNEL_SERIAL_H

#define SERIAL_COM1 0x3F8
#define SERIAL_COM2 0x2F8
#define SERIAL_COM3 0x3E8
#define SERIAL_COM4 0x2E8

#include <stdint.h>

/**
 * Initialize the serial port.
 * @param port The port to initialize.
 * @param baud The baud rate to use. Must be a factor of 115200.
 * @return 0 on success, -1 on failure.
 */
int serial_init(uint16_t port, int baud);

/**
 * Send a character to the serial port.
 * @param port The port to send the character to.
 * @param data The character to send.
 * @return 0 on success, -1 on failure.
 */
int serial_send(uint16_t port, char data);

/**
 * Print a string to the serial port.
 * @param port The port to print the string to.
 * @param str The string to print.
 * @return 0 on success, -1 on failure.
 */
int serial_print(uint16_t port, char* str);

/**
 * Print a string to the serial port, followed by a newline.
 * @param port The port to print the string to.
 * @param str The string to print.
 * @return 0 on success, -1 on failure.
 */
int serial_println(uint16_t port, char* str);

#endif