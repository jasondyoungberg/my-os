#pragma once
#include <stdint.h>

#define PAGEFLAG_P (1 << 0)   // Present
#define PAGEFLAG_RW (1 << 1)  // Read/Write
#define PAGEFLAG_US (1 << 2)  // User/Supervisor
#define PAGEFLAG_PWT (1 << 3) // Write-Through
#define PAGEFLAG_PCD (1 << 4) // Cache Disable
#define PAGEFLAG_A (1 << 5)   // Accessed
#define PAGEFLAG_D (1 << 6)   // Dirty
#define PAGEFLAG_PAT (1 << 7) // Page Attribute Table
#define PAGEFLAG_G (1 << 8)   // Global

void* hhdm(uint64_t);
void map_init();
uint64_t map_page(void* ptr, uint64_t flags);
