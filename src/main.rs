use std::{sync::mpsc, thread};

// type aliasing
// makes code more readable
type Job = Box<dyn FnOnce() + Send + 'static>;

fn hi_there() {
    println!("Hi there!");
}

fn main() {
    let (tx, rx) = mpsc::channel::<Job>();
    let handle = thread::spawn(move || {
        while let Ok(job) = rx.recv() {
            job();
        }
    });

    let job = || println!("Hello from a closure");
    let job2 = || {
        for i in 0..10 {
            println!("{i}");
        }
    };

    tx.send(Box::new(job)).unwrap();
    tx.send(Box::new(job2)).unwrap();
    tx.send(Box::new(hi_there)).unwrap();
    tx.send(Box::new(|| println!("I'm in a Box"))).unwrap();

    handle.join().unwrap();
}
