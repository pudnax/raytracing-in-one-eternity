use rand::prelude::*;

fn main() {
    let n = 1000_000;
    let mut sum = 0.0;

    let mut rng = thread_rng();
    for _ in 0..n {
        let x = f64::powf(rng.gen_range(0., 2.), 1. / 3.);
        sum += x * x / pdf(x);
    }

    println!("I = {}", sum / n as f64);
}

#[inline]
fn pdf(x: f64) -> f64 {
    3. * x * x / 8.
}
