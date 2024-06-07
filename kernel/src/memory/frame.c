#include "memory/frame.h"

#include "common/log.h"
#include "common/memory.h"
#include "common/spinlock.h"
#include "memory/mapping.h"
#include "requests.h"
#include <limine.h>
#include <stdatomic.h>

typedef struct FrameNode {
    struct FrameNode* next;
    uint64_t phys;
    int frames;
};

static struct FrameNode* g_head = 0;
static atomic_ulong g_lock = 0;

void init_frame_alloc() {
    spin_acquire(&g_lock);

    struct limine_memmap_response mmap = *memmap_request.response;
    for (unsigned int i = 0; i < mmap.entry_count; i++) {
        struct limine_memmap_entry* entry = mmap.entries[i];
        if (entry->type != LIMINE_MEMMAP_USABLE)
            continue;

        struct FrameNode* node = hhdm(entry->base);
        node->next = g_head;
        node->phys = entry->base;
        node->frames = entry->length / 4096;
        g_head = node;
    }

    spin_release(&g_lock);
}

uint64_t frame_alloc() {
    spin_acquire(&g_lock);

    if (g_head == NULL)
        panic("out of frames");

    uint64_t result = g_head->phys;

    if (g_head->frames > 1) {
        struct FrameNode* new_head = (uint64_t)g_head + 4096;
        new_head->frames = g_head->frames - 1;
        new_head->phys = g_head->phys + 4096;

        g_head = new_head;
    } else {
        g_head = g_head->next;
    }

    spin_release(&g_lock);
    return result;
}

uint64_t frame_alloc_zero() {
    uint64_t result = frame_alloc();
    memset(hhdm(result), 0, 4096);
    return result;
}

void frame_free(uint64_t frame) {
    log_info("free %d", g_lock);
    spin_acquire(&g_lock);

    struct FrameNode* node = hhdm(frame);
    node->next = g_head;
    node->phys = frame;
    node->frames = 1;
    g_head = node;

    spin_release(&g_lock);
}
