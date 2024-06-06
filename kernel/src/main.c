#include "debugcon.h"
#include "display.h"
#include "io.h"
#include "memory/alloc.h"
#include "memory/frame_alloc.h"
#include "memory/memops.h"
#include "panic.h"
#include "requests.h"
#include "structures/gdt.h"
#include "structures/idt.h"
#include <limine.h>
#include <stdbool.h>

void smp_start(struct limine_smp_info *cpu);

// The following will be our kernel's entry point.
// If renaming _start() to something else, make sure to change the
// linker script accordingly.
void _start(void) {
    kprintf("Hello, world!\n");

    // Ensure the bootloader actually understands our base revision (see spec).
    if (LIMINE_BASE_REVISION_SUPPORTED == false)
        panic("Bootloader revision not supported");

    // Ensure we got a framebuffer.
    if (framebuffer_request.response == NULL ||
        framebuffer_request.response->framebuffer_count < 1)
        panic("No framebuffer available");

    if (smp_request.response == NULL)
        panic("SMP request failed");

    init_frame_alloc();

    struct limine_smp_info *bsp_cpu;
    for (unsigned int i = 0; i < smp_request.response->cpu_count; i++) {
        struct limine_smp_info *cpu = smp_request.response->cpus[i];
        if (cpu->lapic_id == smp_request.response->bsp_lapic_id) {
            bsp_cpu = cpu;
        } else {
            cpu->goto_address = &smp_start;
        }
    }

    if (bsp_cpu != NULL) {
        smp_start(bsp_cpu);
    } else {
        panic("no bsp");
    }

    for (;;)
        __asm__("hlt");
}

void smp_start(struct limine_smp_info *cpu) {
    kprintf("CPU %d started\n", cpu->lapic_id);

    gdt_init();
    idt_init();

    for (;;)
        for (int t = 0; t < 256; t++)
            for (int y = 0; y < display_height(); y++)
                for (int x = 0; x < display_width(); x++)
                    set_pixel(x, y, (struct Color){x, y, t});

    for (;;)
        __asm__("hlt");
}
