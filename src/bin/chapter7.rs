#![cfg(target_os = "linux")]

use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::AtomicU32,
};

pub fn wait(a: &AtomicU32, expected: u32) {
    // Refer to the futex (2) man page for the syscall signature.
    unsafe {
        libc::syscall(
            libc::SYS_futex,                    // The futex syscall.
            a as *const AtomicU32,              // The atomic to operate on.
            libc::FUTEX_WAIT,                   // The futex operation.
            expected,                           // The expected value.
            std::ptr::null::<libc::timespec>(), // No timeout.
        );
    }
}
pub struct Mutex<T> {
    state: AtomicU32,
    value: UnsafeCell<T>,
}
unsafe impl<T> Sync for Mutex<T> where T: Send {}
pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}
unsafe impl<T> Sync for MutexGuard<'_, T> where T: Send {}
impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.mutex.value.get() }
    }
}
impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.value.get() }
    }
}

impl<T> Mutex<T> {
    pub const fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(0),
            value: UnsafeCell::new(value),
        }
    }
    pub fn lock(&self) -> MutexGuard<T> {
        while self.state.swap(1, std::sync::atomic::Ordering::Acquire) == 1 {
            wait(&self.state, 1);
        }
        MutexGuard { mutex: self }
    }
}

fn main() {
    // Add your code here.
}
