#pragma once
#include <stdatomic.h>

/// @brief Acquire a spinlock.
/// @param lock The lock to acquire.
/// @note This function will disable interrupts.
void spin_acquire(atomic_ulong* lock);

/// @brief Release a spinlock.
/// @param lock The lock to release.
/// @note This function will enable interrupts if they were enabled when the
/// lock was acquired.
void spin_release(atomic_ulong* lock);
