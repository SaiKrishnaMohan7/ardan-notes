use std::sync::Mutex;

fn main() {
    let my_shared = Mutex::new(0);

    // deadlock!
    // lock is released when lock goes out of scope
    let lock = my_shared.lock().unwrap();
    let another_lock = my_shared.lock().unwrap();
}
