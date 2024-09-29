use std::{cell::UnsafeCell, mem::MaybeUninit, sync::atomic::AtomicBool};

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}
unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    // constって何？
    pub const fn new() -> Self {
        Channel {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }
    pub unsafe fn send(&self, message: T) {
        (*self.message.get()).write(message);
        self.ready.store(true, std::sync::atomic::Ordering::Release);
    }
    pub fn is_ready(&self) -> bool {
        self.ready.load(std::sync::atomic::Ordering::Acquire)
    }
    pub unsafe fn receive(&self) -> T {
        (*self.message.get()).assume_init_read()
    }
}
fn main() {}
