use std::thread;

use rand::{distributions::{Distribution, Uniform}, thread_rng};

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
        .map(|(inside, outside)| {
            (4.0 * (inside as f64)) / (inside + outside) as f64
        })
        .sum::<f64>();

    let pi = res / num_cpus::get() as f64;
    println!("π ≈ {}", pi);
}
