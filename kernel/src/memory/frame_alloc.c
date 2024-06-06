#include "frame_alloc.h"

#include "requests.h"
#include "spinlock.h"
#include "structures/paging.h"
#include <limine.h>
#include <stdatomic.h>

struct FrameNode {
    struct FrameNode *next;
    uint64_t phys;
    int frames;
};

static struct FrameNode *head = 0;
static atomic_int *lock = 0;

void init_frame_alloc() {
    spin_acquire(lock);

    struct limine_memmap_response mmap = *memmap_request.response;
    for (unsigned int i = 0; i < mmap.entry_count; i++) {
        struct limine_memmap_entry *entry = mmap.entries[i];
        if (entry->type != LIMINE_MEMMAP_USABLE)
            continue;

        struct FrameNode *node = convert_phys_to_virt(entry->base);
        node->next = head;
        node->phys = entry->base;
        node->frames = entry->length / 4096;
        head = node;
    }

    spin_release(lock);
}

uint64_t frame_alloc() {
    spin_acquire(lock);

    if (head == NULL)
        return 0;

    uint64_t result = head->phys;

    if (head->frames > 1) {
        head->frames -= 1;
        head->phys += 4096;
    } else {
        head = head->next;
    }

    return result;

    spin_release(lock);
}

void frame_free(uint64_t frame) {
    spin_acquire(lock);

    struct FrameNode *node = convert_phys_to_virt(frame);
    node->next = head;
    node->phys = frame;
    node->frames = 1;
    head = node;

    spin_release(lock);
}
