use std::thread;

fn hello_thread(n: u32) {
    println!("Hello from thread {n}");
}

fn main() {
    println!("Hello from the main thread");

    let mut thread_handles = Vec::new();

    for i in 0..5 {
        let thread_handle = thread::spawn(move || hello_thread(i));
        thread_handles.push(thread_handle);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }
}
