#include "requests.h"

#include <limine.h>

// Set the base revision to 2, this is recommended as this is the latest
// base revision described by the Limine boot protocol specification.
// See specification for further info.

__attribute__((used, section(".requests"))) volatile LIMINE_BASE_REVISION(2)

    // The Limine requests can be placed anywhere, but it is important that
    // the compiler does not optimise them away, so, usually, they should
    // be made volatile or equivalent, _and_ they should be accessed at least
    // once or marked as used with the "used" attribute as done here.

    __attribute__((
        used, section(".requests"))) volatile struct limine_framebuffer_request
    framebuffer_request = {.id = LIMINE_FRAMEBUFFER_REQUEST, .revision = 0};

__attribute__((used, section(".requests"))) volatile struct limine_smp_request
    smp_request = {.id = LIMINE_SMP_REQUEST, .revision = 0};

__attribute__((used, section(".requests"))) volatile struct limine_hhdm_request
    hhdm_request = {.id = LIMINE_HHDM_REQUEST, .revision = 0};

__attribute__((used,
               section(".requests"))) volatile struct limine_memmap_request
    memmap_request = {.id = LIMINE_MEMMAP_REQUEST, .revision = 0};

// Finally, define the start and end markers for the Limine requests.
// These can also be moved anywhere, to any .c file, as seen fit.

__attribute__((used,
               section(".requests_start_"
                       "marker"))) static volatile LIMINE_REQUESTS_START_MARKER

    __attribute__((
        used,
        section(
            ".requests_end_marker"))) static volatile LIMINE_REQUESTS_END_MARKER
