use std::{
    sync::atomic::{AtomicU64, AtomicUsize, Ordering::Relaxed},
    thread,
    time::{Duration, Instant},
};

fn process_item(_: usize) {
    thread::sleep(Duration::from_millis(123));
}
fn report_example() {
    // 2.2.1
    let num_done = &AtomicUsize::new(0);
    thread::scope(|s| {
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    process_item(t * 25 + i);
                    num_done.fetch_add(1, Relaxed);
                }
            });
        }
        loop {
            print!("loop");
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Working ... {n} / 100");
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn stats_example() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);
    thread::scope(|s| {
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    let start = Instant::now();
                    process_item(t * 25 + i);
                    let time_taken = start.elapsed().as_micros() as u64;
                    num_done.fetch_add(1, Relaxed);
                    total_time.fetch_add(time_taken, Relaxed);
                    max_time.fetch_max(time_taken, Relaxed);
                }
            });
        }
        loop {
            let total_time = Duration::from_micros(total_time.load(Relaxed));
            let max_time = Duration::from_micros(max_time.load(Relaxed));
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            if n == 0 {
                println!("Working started");
            } else {
                println!(
                    "{n} / 100 Done {:?} average, {:?} peak",
                    total_time / n as u32,
                    max_time
                );
            }
        }
    });
    println!("Done!");
}

fn main() {
    // report_example();
    stats_example();
}
