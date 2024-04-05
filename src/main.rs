use std::{sync::{atomic::{AtomicU64, Ordering}, Arc}, thread};

use num_cpus::get;
use rand::{rngs::OsRng, Rng};




fn main() {
    let mut rng = OsRng;

    let in_circle = Arc::new(AtomicU64::new(0));
    let out_circle = Arc::new(AtomicU64::new(0));

    let iters: u64 = std::env::args().nth(1).unwrap().parse().unwrap();

    let iters_per_thread = iters / num_cpus::get() as u64;
    let mut handles = Vec::new();
    for _ in 0..num_cpus::get() {
        let i = in_circle.clone();
        let o = out_circle.clone();
        let handle = thread::spawn(move || {
            for _ in 0..iters_per_thread {
                let x = rng.gen_range(-1f64..1f64);
                let y = rng.gen_range(-1f64..1f64);

                if x * x + y * y <= 1.0 {
                    i.fetch_add(1, Ordering::Relaxed);
                } else {
                    o.fetch_add(1, Ordering::Relaxed);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        // wait
        handle.join().unwrap();
    }

    let actual_inside = in_circle.fetch_or(0, Ordering::Release) as f64;
    let actual_outside = out_circle.fetch_or(0, Ordering::Release) as f64;

    let pi = 4.0 * actual_inside / (actual_inside + actual_outside);
    println!("π ≈ {}", pi);
}
