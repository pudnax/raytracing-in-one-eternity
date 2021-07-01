use rand::prelude::*;

fn main() {
    let mut inside_circle = 0;
    let mut inside_circle_stratified = 0;
    let sqrt_n = 10000;

    let mut rng = thread_rng();

    for i in 0..sqrt_n {
        for j in 0..sqrt_n {
            let (x, y) = (rng.gen_range(-1. ..1.), rng.gen_range(-1. ..1.));

            if (x * x + y * y) < 1. {
                inside_circle += 1;
            }

            let x = 2. * ((i as f64 + rng.gen_range(0. ..1.)) / sqrt_n as f64) - 1.;
            let y = 2. * ((j as f64 + rng.gen_range(0. ..1.)) / sqrt_n as f64) - 1.;

            if (x * x + y * y) < 1. {
                inside_circle_stratified += 1;
            }
        }
    }

    let n = sqrt_n * sqrt_n;
    println!(
        "Regular estimate of Pi = {}",
        4. * inside_circle as f64 / n as f64
    );
    println!(
        "Stratified estimate of Pi = {}",
        4. * inside_circle_stratified as f64 / n as f64
    );
}
