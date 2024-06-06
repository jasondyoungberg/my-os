#ifndef SPINLOCK_H_
#define SPINLOCK_H_

#include <stdatomic.h>

/// @brief Acquire a spinlock.
/// @param lock The lock to acquire.
/// @note This function will disable interrupts.
void spin_acquire(atomic_int *lock);

/// @brief Release a spinlock.
/// @param lock The lock to release.
/// @note This function will enable interrupts if they were enabled when the
/// lock was acquired.
void spin_release(atomic_int *lock);

#endif
