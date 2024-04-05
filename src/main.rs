use rand::{rngs::OsRng, Rng};




fn main() {
    let mut rng = OsRng;

    let mut in_circle: f64 = 0.0;
    let mut out_circle: f64 = 0.0;

    let iters: u64 = std::env::args().nth(1).unwrap().parse().unwrap();
    for _ in 0..iters {
        let x = rng.gen_range(-1f64..1f64);
        let y = rng.gen_range(-1f64..1f64);

        if x * x + y * y <= 1.0 {
            in_circle += 1.0;
        } else {
            out_circle += 1.0;
        }
    }

    let pi = 4.0 * in_circle / (in_circle + out_circle);
    println!("π ≈ {}", pi);
}
