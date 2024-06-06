#pragma once
#include <stdint.h>

void port_write8(uint16_t port, uint8_t val);
void port_write16(uint16_t port, uint16_t val);
void port_write32(uint16_t port, uint32_t val);
uint8_t port_read8(uint16_t port);
uint16_t port_read16(uint16_t port);
uint32_t port_read32(uint16_t port);

void port_wait();
