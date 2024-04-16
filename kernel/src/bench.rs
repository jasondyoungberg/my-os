use alloc::vec::Vec;
use x86_64::instructions::interrupts::without_interrupts;

use crate::println;

pub fn profile<F: Fn()>(msg: &'static str, iters: usize, code: F) {
    let mut results = (0..iters)
        .map(|_| {
            without_interrupts(|| {
                let start = unsafe { core::arch::x86_64::_rdtsc() };
                code();
                let end = unsafe { core::arch::x86_64::_rdtsc() };
                end - start
            })
        })
        .collect::<Vec<u64>>();
    results.sort_unstable();

    let min = results[0];
    let q1 = results[iters / 4];
    let median = results[iters / 2];
    let q3 = results[iters * 3 / 4];
    let max = results[iters - 1];

    println!("{msg}: {min} / {q1} / {median} / {q3} / {max}");
}
