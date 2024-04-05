use std::thread;

use rand::{distributions::{Distribution, Uniform}, rngs::OsRng, thread_rng, Rng};

fn main() {
    let uniform = Uniform::new_inclusive(-1f64, 1f64);

    let iters: u64 = std::env::args().nth(1).unwrap().parse().unwrap();

    let iters_per_thread = iters / num_cpus::get() as u64;
    let mut handles = Vec::new();
    for _ in 0..num_cpus::get() {
        let handle = thread::spawn(move || {
            let mut rng = thread_rng();
            let mut num_in: u64  = 0;
            let mut num_out: u64 = 0;
            for _ in 0..iters_per_thread {
                let x = uniform.sample(&mut rng);
                let y = uniform.sample(&mut rng);

                if x * x + y * y <= 1.0 {
                    num_in += 1;
                } else {
                    num_out += 1;
                }
            }
            (num_in, num_out) // return result from thread
        });
        handles.push(handle);
    }

    let res = handles.into_iter()
        .map(|h| h.join().unwrap())
        .fold((0, 0), |(a, b), (c, d)| (a + c, b + d));

    let actual_inside = res.0 as f64;
    let actual_outside = res.1 as f64;

    let pi = 4.0 * actual_inside / (actual_inside + actual_outside);
    println!("π ≈ {}", pi);
}
