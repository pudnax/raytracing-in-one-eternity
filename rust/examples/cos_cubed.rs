use rand::prelude::*;

fn main() {
    let n = 1000_000;
    let mut sum = 0.0;
    let mut rng = thread_rng();
    for _ in 0..200 {
        let r1 = rng.gen::<f64>();
        let r2 = rng.gen::<f64>();
        let _x = (2. * 3.141592 * r1).cos() * 2. * (r2 * (1. - r2)).sqrt();
        let _y = (2. * 3.141592 * r1).sin() * 2. * (r2 * (1. - r2)).sqrt();
        let z = 1. - r2;
        sum += z * z * z / (1. / (2. * 3.141592));
    }

    println!("Pi/2 = {}", 3.141592 / 2.);
    println!("Estimate = {}", sum / n as f64);
}
