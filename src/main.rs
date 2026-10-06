use std::{io::stdin, thread};

fn parkable_thread(n: u32) {
    loop {
        thread::park();
        println!("Thread {n} is unparked, briefly");
    }
}

fn read_line() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

fn main() {
    let mut thread_handles = Vec::new();
    for i in 0..10 {
        let handle = thread::spawn(move || parkable_thread(i));
        thread_handles.push(handle);
    }

    loop {
        println!("Thread to unpark: ");
        let input = read_line();
        if input == "q" {
            break;
        }

        if let Ok(number) = input.parse::<usize>() {
            if number < 10 {
                thread_handles[number].thread().unpark();
            }
        }
    }
}
