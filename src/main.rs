use std::thread;

fn main() {
    const N_THREADS: usize = 8;
    let to_add: Vec<u32> = (0..5000).collect();
    let mut thread_handles = Vec::new();
    let chunks = to_add.chunks(N_THREADS);

    for chunk in chunks {
        let my_chunk = chunk.to_owned(); // since the thread that will be doing the work needs own this chunk
        let handle = thread::spawn(move || my_chunk.iter().sum::<u32>());
        thread_handles.push(handle);
    }

    let mut sum = 0;
    for h in thread_handles {
        sum += h.join().unwrap();
    }
    println!("Sum is {sum}");
}
