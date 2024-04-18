use alloc::vec::Vec;
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
static WAKERS: Mutex<Vec<(Id, u64, AtomicWaker)>> = Mutex::new(Vec::new());

pub fn add_time(time: Duration) {
    let ns = u64::try_from(time.as_nanos()).unwrap();
    NOW.fetch_add(ns, Ordering::Relaxed);

    MAIN_WAKER.wake();
}

pub async fn main_task() {
    poll_fn(|cx| {
        MAIN_WAKER.register(cx.waker());

        let now = NOW.load(Ordering::Relaxed);

        for (_, wake_time, waker) in WAKERS.lock().iter_mut() {
            if now >= *wake_time {
                waker.wake();
            }
        }

        Poll::<()>::Pending
    })
    .await;
}

pub async fn delay(time: Duration) {
    let ns = u64::try_from(time.as_nanos()).unwrap();
    let wake_time = NOW.load(Ordering::Relaxed) + ns;
    let waker = AtomicWaker::new();
    let id = Id::new();

    WAKERS.lock().push((id, wake_time, waker));

    poll_fn(|cx| {
        if NOW.load(Ordering::Relaxed) >= wake_time {
            return Poll::Ready(());
        }

        let mut wakers = WAKERS.lock();

        let (idx, (_, _, waker)) = &wakers
            .iter()
            .enumerate()
            .find(|(_, (other_id, _, _))| id == *other_id)
            .unwrap();

        waker.register(cx.waker());

        if NOW.load(Ordering::Relaxed) >= wake_time {
            waker.take();
            wakers.remove(*idx);
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    })
    .await
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Id(u64);

impl Id {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);

        Self(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}
