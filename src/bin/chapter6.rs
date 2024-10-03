use std::{
    ops::Deref,
    ptr::NonNull,
    sync::atomic::{fence, AtomicUsize},
};

fn main() {}

struct ArcData<T> {
    ref_count: AtomicUsize,
    data: T,
}

pub struct MyArc<T> {
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

impl<T> MyArc<T> {
    pub fn new(data: T) -> MyArc<T> {
        MyArc {
            ptr: NonNull::from(Box::leak(Box::new(ArcData {
                ref_count: AtomicUsize::new(1),
                data,
            }))),
        }
    }
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }
    pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
        if arc
            .data()
            .ref_count
            .load(std::sync::atomic::Ordering::Relaxed)
            == 1
        {
            fence(std::sync::atomic::Ordering::Acquire);
            unsafe { Some(&mut arc.ptr.as_mut().data) }
        } else {
            None
        }
    }
}

impl<T> Deref for MyArc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.data().data
    }
}

impl<T> Clone for MyArc<T> {
    fn clone(&self) -> Self {
        self.data()
            .ref_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        MyArc { ptr: self.ptr }
    }
}

impl<T> Drop for MyArc<T> {
    fn drop(&mut self) {
        if self
            .data()
            .ref_count
            .fetch_sub(1, std::sync::atomic::Ordering::Release)
            == 1
        {
            fence(std::sync::atomic::Ordering::Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
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
            NUM_DROPS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
    }
    let x = MyArc::new(("Hello", DetectDrop));
    let y = x.clone();
    let t = std::thread::spawn(move || {
        assert_eq!(x.0, "Hello");
    });
    assert_eq!(y.0, "Hello");
    t.join().unwrap();
    assert_eq!(NUM_DROPS.load(std::sync::atomic::Ordering::Relaxed), 0);
    drop(y);
    assert_eq!(NUM_DROPS.load(std::sync::atomic::Ordering::Relaxed), 1);
}
