use std::sync::Mutex;

fn main() {
    let my_shared = Mutex::new(0);

    let lock = my_shared.lock().unwrap();
    if let Ok(_lock) = my_shared.try_lock() {
        println!("got the lock");
    } else {
        println!("No lock");
    };
}
