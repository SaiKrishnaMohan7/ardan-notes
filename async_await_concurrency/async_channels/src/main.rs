use std::{sync::mpsc, time::Duration};

enum Command {
    Print(String),
}

#[tokio::main]
async fn main() {
    // Spawn a command thread for "heavy lifting"
    let (tx, rx) = mpsc::channel::<Command>();
    let (tx_reply, mut rx_reply) = tokio::sync::mpsc::channel::<String>(10);
    // handle to running tokio instance
    // allows to drop into tokio from outside and inject async code into the task queue
    let handle = tokio::runtime::Handle::current();

    std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::Print(s) => {
                    println!("{s} received in channel");
                    let tx_reply_clone = tx_reply.clone();
                    handle.spawn(async move {
                        println!("sending Print command string to async channel");
                        tx_reply_clone.send(s).await.unwrap();
                    });
                }
            }
        }
    });

    // Receive messges
    tokio::spawn(async move {
        while let Some(reply) = rx_reply.recv().await {
            println!("Received {reply} in async channel");
        }
    });

    // Launch the async sender
    let mut counter = 0;
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        tx.send(Command::Print(format!("Hello {counter}"))).unwrap();
        counter += 1;
    }
}

// A pattern like this could be used for making use of the same server
// and sending async work, like building a pdf, to the async worker
