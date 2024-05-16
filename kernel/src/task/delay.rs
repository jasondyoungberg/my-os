use alloc::collections::BTreeMap;
use core::{
    future::poll_fn,
    sync::atomic::{AtomicU64, Ordering},
    task::Poll,
    time::Duration,
};
use futures::task::AtomicWaker;
use spin::Mutex;

static NOW: AtomicU64 = AtomicU64::new(0);

static MAIN_WAKER: AtomicWaker = AtomicWaker::new();

static SLEEP_WAKERS: Mutex<BTreeMap<u64, AtomicWaker>> = Mutex::new(BTreeMap::new());

pub fn add_time(delta: Duration) {
    let delta_ns = u64::try_from(delta.as_nanos()).unwrap();

    NOW.fetch_add(delta_ns, Ordering::Relaxed);

    MAIN_WAKER.wake();
}

pub async fn main_task() {
    poll_fn(|cx| {
        MAIN_WAKER.register(cx.waker());

        let now = NOW.load(Ordering::Relaxed);
        let mut sleep_wakers = SLEEP_WAKERS.lock();

        while let Some(sleep_wakers_entry) = sleep_wakers.first_entry() {
            let wake_time = sleep_wakers_entry.key();
            if *wake_time > now {
                break;
            }

            let (_, sleep_waker) = sleep_wakers_entry.remove_entry();
            sleep_waker.wake();
        }

        Poll::<()>::Pending
    })
    .await;
}

pub async fn delay(duration: Duration) {
    let duration_ns = u64::try_from(duration.as_nanos()).unwrap();
    let mut wake_time = NOW.load(Ordering::Relaxed) + duration_ns;
    let this_waker = AtomicWaker::new();

    let mut sleep_wakers = SLEEP_WAKERS.lock();

    // only one waker can exist per wake time,
    // so we need to increment the wake time until we find an empty slot
    while sleep_wakers.contains_key(&wake_time) {
        wake_time += 1;
    }

    sleep_wakers.insert(wake_time, this_waker);

    drop(sleep_wakers);

    poll_fn(|cx| {
        // fast path
        if NOW.load(Ordering::Relaxed) >= wake_time {
            return Poll::Ready(());
        }

        let mut sleep_wakers = SLEEP_WAKERS.lock();
        let this_waker = sleep_wakers.get(&wake_time).unwrap();

        this_waker.register(cx.waker());

        // check if the time has already passed to avoid race conditions
        if NOW.load(Ordering::Relaxed) >= wake_time {
            this_waker.take();
            sleep_wakers.remove(&wake_time);
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    })
    .await
}
