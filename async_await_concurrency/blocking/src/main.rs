use std::{thread, time::Duration};

async fn hello_delay(task: u64, time: u64) {
    println!("Task {task} has started");
    thread::sleep(Duration::from_millis(time)); // makes the whole tokio runtime go to sleep (if current_thread enabled OR if numBlockingTasks > numThreads); Block OS thread!
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
    // join! without spawn runs all three futures inside one task
    // and a task runs on one thread at a time. When that thread hits thread::sleep,
    // everything in the task stops, while the other workers have nothing to pick up.
    tokio::join!(
        hello_delay(1, 500),
        hello_delay(2, 1000),
        hello_delay(3, 1500)
    );
}

// What to take away:
// - thread::sleep blocks the OS thread; tokio::time::sleep(...).await pauses only the task.
// - Within one task, futures take turns only at .await.
// - In async code, never block a thread. For blocking work you can't avoid, use tokio::task::spawn_blocking, which runs it on a separate thread pool meant for blocking work.
