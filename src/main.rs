use std::{
    sync::atomic::{AtomicI32, Ordering},
    thread,
    time::Duration,
};

use thread_priority::{ThreadPriority, set_current_thread_priority};

static LOW_COUNT: AtomicI32 = AtomicI32::new(0);
static MED_COUNT: AtomicI32 = AtomicI32::new(0);
static HIGH_COUNT: AtomicI32 = AtomicI32::new(0);

fn low_priority() {
    set_current_thread_priority(ThreadPriority::Min).unwrap();
    loop {
        LOW_COUNT.fetch_add(1, Ordering::Relaxed);
        // I am done with this iteration you can schedule another thread!
        thread::yield_now();
    }
}

fn reg_priority() {
    loop {
        MED_COUNT.fetch_add(1, Ordering::Relaxed);
        // I am done with this iteration you can schedule another thread!
        thread::yield_now();
    }
}

fn high_priority() {
    set_current_thread_priority(ThreadPriority::Max).unwrap();
    loop {
        HIGH_COUNT.fetch_add(1, Ordering::Relaxed);
        // I am done with this iteration you can schedule another thread!
        thread::yield_now();
    }
}

fn main() {
    thread::spawn(low_priority);
    thread::spawn(reg_priority);
    thread::spawn(high_priority);

    thread::sleep(Duration::from_secs(5));
    println!("Low: {}", LOW_COUNT.load(Ordering::Relaxed));
    println!("Reg: {}", MED_COUNT.load(Ordering::Relaxed));
    println!("High: {}", HIGH_COUNT.load(Ordering::Relaxed));
}
