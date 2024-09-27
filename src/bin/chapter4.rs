use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

pub struct SpinLock {
    locked: AtomicBool,
}
impl SpinLock {
    pub const fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
        }
    }
    pub fn lock(&self) {
        while self.locked.swap(true, Ordering::Acquire) {
            std::hint::spin_loop();
        }
    }
    pub fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}
fn main() {}
