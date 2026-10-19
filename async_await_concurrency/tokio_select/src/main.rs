use std::time::Duration;

async fn do_work() {
    tokio::time::sleep(Duration::from_secs(2)).await;
}

async fn timeout(seconds: f32) {
    tokio::time::sleep(Duration::from_secs_f32(seconds)).await;
}

#[tokio::main]
async fn main() {
    // Ability to choose from which future finishes first
    // other is cancelled BUT state may NOT be reverted like a db change
    tokio::select! {
        _ = do_work() => println!("do_work finshed first"),
        _ = timeout(1.0) => println!("timeout finished first")
    }
}
