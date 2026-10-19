use std::sync::Mutex;

static COUNTER: Mutex<u32> = Mutex::new(0);

async fn add_one(n: u32) -> u32 {
    n + 1
}

async fn increment() {
    let mut counter = COUNTER.lock().unwrap();
    *counter = add_one(*counter).await;
}

#[tokio::main]
async fn main() {
    tokio::join!(increment(), increment(), increment());
    println!("COUNTER = {}", *COUNTER.lock().unwrap());
}
