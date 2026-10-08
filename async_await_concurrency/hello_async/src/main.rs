use futures::{executor::block_on, join};

async fn say_hello() {
    println!("Hello, world!");
    another().await;

    join!(another(), bye());
}

async fn another() {
    println!("Wallahi!")
}

async fn bye() {
    println!("Bue");
}

fn main() {
    // handing control to the async runtime from main
    block_on(say_hello());
}
