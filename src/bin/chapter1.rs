use std::cell::Cell;
use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use std::thread;
use std::time::Duration;

// 1.5.1 Cell
// 共有参照を通した変更を許可する。
// 未定義動作を避けるために、値をコピーして取り出すことと、全体を他の値で置き換えることしかできない。
#[allow(dead_code)]
fn f(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        // this part may happen. a, bが同じものなら、b.setによって、aが変わるから。
    }
}

fn test_mutex() {
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut gurad = n.lock().unwrap();
                for _ in 0..100 {
                    *gurad += 1;
                }
            });
        }
    });
    assert_eq!(n.into_inner().unwrap(), 1000);
}

fn test_condvar() {
    let queue = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();
    thread::scope(|s| {
        s.spawn(|| loop {
            let mut q = queue.lock().unwrap();
            let item = loop {
                if let Some(item) = q.pop_front() {
                    break item;
                } else {
                    q = not_empty.wait(q).unwrap();
                }
            };
            drop(q);
            dbg!(item);
        });
        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            not_empty.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    })
}

// つづきは1.7.3
fn main() {
    test_mutex();
    test_condvar();
}
