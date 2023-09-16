use rand::Rng;
use std::thread::spawn;

/// A simulation of the coupon collector's problem.

/// The number of coupons to collect.
const N: usize = 6;

/// The number of trials to run per thread.
const TRIALS_PER_THREAD: usize = 100;

/// The number of threads to use.
const THREADS: usize = 4;

/// The total number of trials to run. This is just `TRIALS_PER_THREAD * THREADS`, but it's nice to have a constant for it.
const TOTAL_TRIALS: usize = TRIALS_PER_THREAD * THREADS;

fn trial() -> usize {
    let mut rng = rand::thread_rng();
    let mut collected = [false; N];
    let mut count = 0;
    while collected.iter().any(|&x| !x) {
        let coupon = rng.gen_range(0..N);
        if !collected[coupon] {
            collected[coupon] = true;
        }
        count += 1;
    }
    count
}
fn main() {
    println!(
        "Running {} trials per thread on {} threads. {} total trials.",
        TRIALS_PER_THREAD, THREADS, TOTAL_TRIALS
    );
    let mut total = 0;
    let mut handles = Vec::new();
    for _ in 0..THREADS {
        handles.push(spawn(|| {
            let mut total = 0;
            for _ in 0..TRIALS_PER_THREAD {
                total += trial();
            }
            total
        }));
    }
    for handle in handles {
        total += handle.join().unwrap();
    }
    println!(
        "Done! Average number of coupons collected: {}",
        total as f64 / TOTAL_TRIALS as f64
    );
}
