use std::thread;

static mut COUNTER: i32 = 0;

fn main() {
    let mut handles = Vec::new();
    for _ in 0..1000 {
        let handle = thread::spawn(|| {
            for _ in 1..1_100 {
                unsafe {
                    // the `unsafe` is added after compiler suggested it
                    COUNTER = COUNTER + 1;

                    // Adding is not a single step work
                    // The number stored in the register is to be read first
                    // then add 1 to it
                    // then write the new value to the register.
                    // When we have multiple threads, each trying to do this, with no
                    // synchronization, a value read by one thread may have already
                    // changed and the thread reading the value is reading the wrong
                    // value.
                    // This is a data race.
                }
            }
        });

        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    unsafe {
        println!("{COUNTER}"); // 2024 Rust doesn't even let this compile
    }
    // this could be compiled in older Rust compiler versions.
    // Gives a different answer every time the code is run in the old compiler version
}
