fn main() {
    // Sized pool
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.spawn(|| println!("Hello from a thread from the pool"));

    // Rayon handles joining!
    pool.scope(|scope| {
        for n in 0..20 {
            scope.spawn(move |_| {
                println!("Hello from scoped thread {n}");
            });
        }
    });

    println!("Hello from the main thread");
}
