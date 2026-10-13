use std::{thread, time::Duration};

async fn hello_delay(task: u64, time: u64) {
    println!("Task {task} has started");
    thread::sleep(Duration::from_millis(time)); // makes the whole tokio runtime go to sleep
    println!("Task {task} has finished");
}
// expected output:
// Task 1 has started
// Task 2 has started
// Task 3 has started
// Task 1 has finished
// Task 2 has finished
// Task 3 has finished

// Actual output with thread::sleep()
// Task 1 has started
// Task 1 has finished
// Task 2 has started
// Task 2 has finished
// Task 3 has started
// Task 3 has finished

// Tokio cooperatively multitasks and not pre-emptively multitask

#[tokio::main]
async fn main() {
    tokio::join!(
        hello_delay(1, 500),
        hello_delay(2, 1000),
        hello_delay(3, 1500)
    );
}
