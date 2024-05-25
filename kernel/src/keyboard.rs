use core::{
    pin::Pin,
    task::{Context, Poll},
};

use crossbeam::queue::ArrayQueue;
use futures::{task::AtomicWaker, Stream};
use spin::Once;

static SCANCODE_QUEUE: Once<ArrayQueue<u8>> = Once::new();

static WAKER: AtomicWaker = AtomicWaker::new();

pub fn init() {
    SCANCODE_QUEUE.call_once(|| ArrayQueue::new(256));
}

pub fn add_scancode(scancode: u8) {
    if let Some(queue) = SCANCODE_QUEUE.get() {
        if queue.push(scancode).is_ok() {
            WAKER.wake();
        } else {
            log::warn!("scancode queue full; dropping scancode");
        }
    } else {
        log::warn!("scancode queue uninitialized");
    }
}

pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    pub const fn new() -> Self {
        Self { _private: () }
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        if let Some(queue) = SCANCODE_QUEUE.get() {
            // fast path
            if let Some(scancode) = queue.pop() {
                return Poll::Ready(Some(scancode));
            }

            WAKER.register(cx.waker());

            queue.pop().map_or(Poll::Pending, |scancode| {
                WAKER.take();
                Poll::Ready(Some(scancode))
            })
        } else {
            // error!("scancode queue uninitialized");
            WAKER.register(cx.waker());
            Poll::Pending
        }
    }
}
