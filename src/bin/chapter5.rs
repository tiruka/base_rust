use std::{cell::UnsafeCell, mem::MaybeUninit, sync::atomic::AtomicBool, thread};

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
    in_use: AtomicBool,
}
unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    // constって何？
    pub const fn new() -> Self {
        Channel {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
            in_use: AtomicBool::new(false),
        }
    }
    pub fn send(&self, message: T) {
        if self.in_use.swap(true, std::sync::atomic::Ordering::Relaxed) {
            panic!("can't send more than one message!");
        }
        unsafe { (*self.message.get()).write(message) };
        self.ready.store(true, std::sync::atomic::Ordering::Release);
    }
    pub fn is_ready(&self) -> bool {
        self.ready.load(std::sync::atomic::Ordering::Relaxed)
    }
    pub fn receive(&self) -> T {
        if !self.ready.swap(false, std::sync::atomic::Ordering::Acquire) {
            panic!("no messages available");
        }
        unsafe { (*self.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() };
        }
    }
}

fn main() {
    let channel = Channel::new();
    let t = thread::current();
    thread::scope(|s| {
        s.spawn(|| {
            channel.send("hello world!");
            t.unpark();
        });
        while !channel.is_ready() {
            thread::park();
        }
        assert_eq!(channel.receive(), "hello world!");
    });
}
