use std::time::Duration;

async fn hello_delay(task: u64, time: u64) {
    println!("Task {task} has started");
    tokio::time::sleep(Duration::from_millis(time)).await; // this future waits; the thread stays free
    //thread::sleep(Duration::from_millis(time));
    println!("Task {task} has finished");
}

// Observe, output looks like a multithreaded, parallely executed, output
// however, it is concurrent not parallel.
// Output:
// Task 1 has started
// Task 2 has started
// Task 3 has started
// Task 1 has finished
// Task 2 has finished
// Task 3 has finished

#[tokio::main]
async fn main() {
    tokio::join!(
        hello_delay(1, 500),
        hello_delay(2, 1000),
        hello_delay(3, 1500)
    );
}

// all three hello_delay calls are futures inside one task, because of join!. So:
// - sleep(...).await pauses that future and hands control back to join!.
// - join! moves on and runs the next future, until it too reaches its sleep.
// - Once all three are waiting, the task pauses and the thread is free for other work.
// - When a timer fires, the runtime wakes the task, and join! resumes the future whose timer it was.
