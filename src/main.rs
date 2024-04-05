use std::sync::atomic::{AtomicU64, Ordering};

use rand::{rngs::OsRng, Rng};




fn main() {
    let mut rng = OsRng;

    let mut in_circle = AtomicU64::new(0);
    let mut out_circle = AtomicU64::new(0);

    let iters: u64 = std::env::args().nth(1).unwrap().parse().unwrap();
    for _ in 0..iters {
        let x = rng.gen_range(-1f64..1f64);
        let y = rng.gen_range(-1f64..1f64);

        if x * x + y * y <= 1.0 {
            in_circle.fetch_add(1, Ordering::Relaxed);
        } else {
            out_circle.fetch_add(1, Ordering::Relaxed);
        }
    }

    let actual_inside = *in_circle.get_mut() as f64;
    let actual_outside = *out_circle.get_mut() as f64;

    let pi = 4.0 * actual_inside / (actual_inside + actual_outside);
    println!("π ≈ {}", pi);
}
