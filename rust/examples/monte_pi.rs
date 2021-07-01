use rand::prelude::*;

fn main() {
    const N: usize = 1000;
    let mut inside_circle = 0;
    let mut rng = rand::thread_rng();
    for _ in 0..N {
        let x = rng.gen_range(-1. ..1.);
        let y = rng.gen_range(-1. ..1.);
        if (x * x + y * y) < 1. {
            inside_circle += 1;
        }
    }
    println!(
        "Estimate of Pi = {:.08}",
        4. * inside_circle as f64 / N as f64
    );
}
