use std::thread;

fn main() {
    const N_THREADS: usize = 8;
    let to_add: Vec<u32> = (0..5000).collect();
    let chunks = to_add.chunks(N_THREADS);
    let sum = thread::scope(|thread_scope| {
        let mut thread_handles = Vec::new();

        for chunk in chunks {
            // Note: No need to create copies with to_owned(); Everything happens in the thread
            let thread_handle = thread_scope.spawn(move || chunk.iter().sum::<u32>());
            thread_handles.push(thread_handle);
        }

        let mut sum: u32 = 0;
        for h in thread_handles {
            sum = sum + h.join().unwrap();
        }

        sum
    });

    println!("Sum: {sum}");
}
