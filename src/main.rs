use std::{sync::mpsc, thread};

// type aliasing
// makes code more readable
type Job = Box<dyn FnOnce() + Send + 'static>;

enum Command {
    Run(Job),
    Quit,
}

fn main() {
    let (tx, rx) = mpsc::channel::<Command>();
    let handle = thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::Run(job) => job(),
                Command::Quit => {
                    println!("Quitting....");
                    break;
                }
            }
        }
    });

    tx.send(Command::Run(Box::new(|| println!("I am in a Box"))))
        .unwrap();
    tx.send(Command::Quit).unwrap();
    handle.join().unwrap();
}
