use core::{
    pin::Pin,
    task::{Context, Poll},
};

use crate::println;
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
        match queue.push(scancode) {
            Ok(()) => WAKER.wake(),
            Err(_) => println!("WARNING: scancode queue full; dropping scancode"),
        }
    } else {
        println!("WARNING: scancode queue uninitialized");
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
        let queue = SCANCODE_QUEUE.get().expect("scancode queue uninitialized");

        if let Some(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(cx.waker());

        queue.pop().map_or(Poll::Pending, |scancode| {
            WAKER.take();
            Poll::Ready(Some(scancode))
        })
    }
}
