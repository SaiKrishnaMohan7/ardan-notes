use std::sync::Mutex;

fn main() {
    let my_shared = Mutex::new(0);

    // using a scope, not too neat
    {
        let lock = my_shared.lock().unwrap();
    }
    let another_lock = my_shared.lock().unwrap();
}
