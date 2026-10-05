use std::{
    sync::atomic::{AtomicI32, Ordering},
    thread,
};

static COUNTER: AtomicI32 = AtomicI32::new(0);

fn main() {
    let mut handles = Vec::new();

    for _ in 1..1_000 {
        let handle = thread::spawn(|| {
            for _ in 1..1_100 {
                // atomic add!
                COUNTER.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("{}", COUNTER.load(Ordering::Relaxed));
}
