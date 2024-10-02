use std::sync::atomic::Ordering::Relaxed;
use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::{atomic::AtomicBool, Arc},
    thread,
};

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

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let a = Arc::new(Channel2 {
        message: UnsafeCell::new(MaybeUninit::uninit()),
        ready: AtomicBool::new(false),
    });
    (Sender { channel: a.clone() }, Receiver { channel: a })
}

struct Channel2<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}
unsafe impl<T> Sync for Channel2<T> where T: Send {}
pub struct Sender<T> {
    channel: Arc<Channel2<T>>,
}
pub struct Receiver<T> {
    channel: Arc<Channel2<T>>,
}

impl<T> Sender<T> {
    pub fn send(self, message: T) {
        unsafe { (*self.channel.message.get()).write(message) };
        self.channel.ready.store(true, Relaxed);
    }
}
impl<T> Receiver<T> {
    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Relaxed)
    }
    pub fn receive(self) -> T {
        if !self.channel.ready.swap(false, Relaxed) {
            panic!("no message available");
        }
        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}
impl<T> Drop for Channel2<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}

fn main() {
    let ch = Channel::new();
    let t = thread::current();
    thread::scope(|s| {
        s.spawn(|| {
            ch.send("hello world!");
            t.unpark();
        });
        while !ch.is_ready() {
            thread::park();
        }
        assert_eq!(ch.receive(), "hello world!");
    });

    // Using Channel2
    thread::scope(|s| {
        let (sender, receiver) = channel();
        let t = thread::current();
        s.spawn(move || {
            sender.send("Hello World");
            t.unpark();
        });
        while !receiver.is_ready() {
            thread::park();
        }
        assert_eq!(receiver.receive(), "Hello World");
    });
}
