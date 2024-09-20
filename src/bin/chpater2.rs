use std::{
    sync::atomic::AtomicUsize,
    sync::atomic::Ordering::Relaxed,
    thread::{self, sleep},
    time::Duration,
};

fn process_item(_item: i32) {
    sleep(Duration::from_secs(2));
}
fn main() {
    let num_done = &AtomicUsize::new(0);
    thread::scope(|s| {
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..2 {
                    process_item(t * 25 + i);
                    num_done.fetch_add(1, Relaxed);
                }
            });
        }
    });

    loop {
        print!("loop");
        let n = num_done.load(Relaxed);
        if n == 100 {
            break;
        }
        println!("Working ... {n} / 100");
        sleep(Duration::from_secs(1));
    }
}
