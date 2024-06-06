#include "paging.h"

#include "requests.h"
#include <stdint.h>

#define FLAG_P (1 << 0)   // Present
#define FLAG_RW (1 << 1)  // Read/Write
#define FLAG_US (1 << 2)  // User/Supervisor
#define FLAG_PWT (1 << 3) // Write-Through
#define FLAG_PCD (1 << 4) // Cache Disable
#define FLAG_A (1 << 5)   // Accessed
#define FLAG_D (1 << 6)   // Dirty
#define FLAG_PS (1 << 7)  // Page Size
#define FLAG_G (1 << 8)   // Global

void *convert_phys_to_virt(uint64_t phys) {
    return (void *)(phys + hhdm_request.response->offset);
}
