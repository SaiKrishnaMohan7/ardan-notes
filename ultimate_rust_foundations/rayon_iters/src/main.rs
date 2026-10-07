use std::time::Instant;

use rayon::{
    iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

fn is_prime(n: u64) -> bool {
    (2..=n / 2).into_par_iter().all(|i| n % i != 0)
}
fn main() {
    let numbers = (0..1_000_000).collect::<Vec<u64>>();
    // rayon even chunks the workload for you
    // rayon creates a thread for every core on the machine on startup
    // let sum = numbers.par_iter().sum::<u64>();

    // println!("Sum: {sum}");

    let now = Instant::now();
    let mut primes = numbers
        .par_iter()
        .filter(|n| is_prime(**n))
        .collect::<Vec<&u64>>();
    primes.par_sort_unstable();
    let elapsed = now.elapsed().as_secs_f32();
    println!("Found {} primes in {} seconds", primes.len(), elapsed);
}
