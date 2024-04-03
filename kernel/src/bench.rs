use crate::{print, println};

pub fn benchmark<T, F, R>(msg: T, code: F) -> R
where
    T: AsRef<str>,
    F: FnOnce() -> R,
{
    x86_64::instructions::interrupts::without_interrupts(|| {
        let start = unsafe { core::arch::x86_64::_rdtsc() };
        let res = code();
        let end = unsafe { core::arch::x86_64::_rdtsc() };

        println!("{}: {} cycles", msg.as_ref(), end - start);
        res
    })
}
