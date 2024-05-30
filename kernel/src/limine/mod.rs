// This code is from https://github.com/limine-bootloader/limine-rs,
// which is licensed under the MIT License.
//
// MIT License
//
// Copyright (c) 2021 Anhad Singh, 2024 Lysander Mealy
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

mod framebuffer;
pub mod memory_map;
pub mod stack_size;

pub use framebuffer::*;
pub use memory_map::*;

use core::{cell::UnsafeCell, ptr::NonNull};

const MAGIC_1: u64 = 0xc7b1dd30df4c8b88;
const MAGIC_2: u64 = 0x0a82e883a194f07b;

#[repr(C)]
pub struct BaseRevision {
    id: [u64; 2],
    revision: UnsafeCell<u64>,
}
impl BaseRevision {
    pub const fn new() -> Self {
        Self {
            id: [0xf9562b2d5c95a6c8, 0x6a7b384944536bdc],
            revision: UnsafeCell::new(1),
        }
    }

    pub fn is_supported(&self) -> bool {
        (unsafe { self.revision.get().read_volatile() }) == 0
    }
}
unsafe impl Sync for BaseRevision {}
unsafe impl Send for BaseRevision {}

#[repr(transparent)]
pub struct Response<T> {
    inner: UnsafeCell<Option<NonNull<T>>>,
}
impl<T> Response<T> {
    pub const fn none() -> Self {
        Self {
            inner: UnsafeCell::new(None),
        }
    }
    pub fn get(&self) -> Option<&T> {
        Some(unsafe { core::ptr::read_volatile(self.inner.get())?.as_ref() })
    }
    pub fn get_mut(&mut self) -> Option<&mut T> {
        Some(unsafe { core::ptr::read_volatile(self.inner.get())?.as_mut() })
    }
}
unsafe impl<T: Sync> Sync for Response<T> {}
unsafe impl<T: Send> Send for Response<T> {}
