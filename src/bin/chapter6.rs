use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::{
    cell::UnsafeCell,
    ops::Deref,
    ptr::NonNull,
    sync::atomic::{fence, AtomicUsize},
};
fn main() {}

struct ArcData<T> {
    data_ref_count: AtomicUsize,
    alloc_ref_count: AtomicUsize,
    data: UnsafeCell<Option<T>>,
}

pub struct MyArc<T> {
    weak: Weak<T>,
}

pub struct Weak<T> {
    ptr: NonNull<ArcData<T>>,
}

// Tは、まずSend と Syncを実装している必要がある。
// そして、MyArcも、SendとSyncをそれぞれimplする必要があるということを示している。
// Sendトレイト: Sendを実装した型は、別のスレッドに安全に送信（move）できることを意味します。/
// Syncトレイト: Syncを実装した型は、複数のスレッドから安全に参照されることができることを意味します（つまり、複数のスレッドが同じインスタンスを共有しても安全であること）。
// SendとSyncのトレイトは通常、安全性がRustコンパイラによって自動的に判断されます。しかし、場合によっては、型の設計によって自動でSendやSyncが実装されないことがあります。
// その場合、開発者が手動で実装を提供し、コンパイラに「この型はスレッドセーフですよ」と伝える必要があります。
// ただし、この手動実装はメモリの安全性に関わるため、unsafeブロックが必要です。unsafeを使う理由は、
// スレッド間でデータが正しく共有されていることや、データ競合が発生しないことを開発者が自分で確認し、保証する必要があるためです
unsafe impl<T: Send + Sync> Send for MyArc<T> {}
unsafe impl<T: Send + Sync> Sync for MyArc<T> {}

unsafe impl<T: Send + Sync> Send for Weak<T> {}
unsafe impl<T: Send + Sync> Sync for Weak<T> {}
impl<T> Weak<T> {
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }
    fn upgrade(&self) -> Option<MyArc<T>> {
        let mut n = self.data().data_ref_count.load(Relaxed);
        loop {
            if n == 0 {
                return None;
            }
            assert!(n < usize::MAX / 2);
            if let Err(e) =
                self.data()
                    .data_ref_count
                    .compare_exchange_weak(n, n + 1, Relaxed, Relaxed)
            {
                n = e;
                continue;
            }
            return Some(MyArc { weak: self.clone() });
        }
    }
}
impl<T> MyArc<T> {
    pub fn new(data: T) -> MyArc<T> {
        MyArc {
            weak: Weak {
                ptr: NonNull::from(Box::leak(Box::new(ArcData {
                    data_ref_count: AtomicUsize::new(1),
                    alloc_ref_count: AtomicUsize::new(1),
                    data: UnsafeCell::new(Some(data)),
                }))),
            },
        }
    }
    pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
        if arc.weak.data().alloc_ref_count.load(Relaxed) == 1 {
            fence(Acquire);
            let arcdata = unsafe { arc.weak.ptr.as_mut() };
            let option = arcdata.data.get_mut();
            let data = option.as_mut().unwrap();
            Some(data)
        } else {
            None
        }
    }
    pub fn downgrade(arc: &Self) -> Weak<T> {
        arc.weak.clone()
    }
}

impl<T> Deref for MyArc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        let ptr = self.weak.data().data.get();
        unsafe { (*ptr).as_ref().unwrap() }
    }
}

impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        if self.data().data_ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }
        Weak { ptr: self.ptr }
    }
}

impl<T> Clone for MyArc<T> {
    fn clone(&self) -> Self {
        let weak = self.weak.clone();
        if weak.data().data_ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }
        MyArc { weak }
    }
}

impl<T> Drop for Weak<T> {
    fn drop(&mut self) {
        if self.data().data_ref_count.fetch_sub(1, Relaxed) == 1 {
            fence(Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

impl<T> Drop for MyArc<T> {
    fn drop(&mut self) {
        if self.weak.data().data_ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            let ptr = self.weak.data().data.get();
            unsafe {
                *ptr = None;
            }
        }
    }
}

#[test]
fn test() {
    static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);
    struct DetectDrop;
    impl Drop for DetectDrop {
        fn drop(&mut self) {
            NUM_DROPS.fetch_add(1, Relaxed);
        }
    }
    let x = MyArc::new(("Hello", DetectDrop));
    let y = x.clone();
    let t = std::thread::spawn(move || {
        assert_eq!(x.0, "Hello");
    });
    assert_eq!(y.0, "Hello");
    t.join().unwrap();
    assert_eq!(NUM_DROPS.load(Relaxed), 0);
    drop(y);
    assert_eq!(NUM_DROPS.load(Relaxed), 1);
}
