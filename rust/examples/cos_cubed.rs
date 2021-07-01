use rand::prelude::*;
use std::f64::consts::PI;

fn main() {
    let n = 1_000_000;
    let mut sum = 0.0;
    let mut rng = thread_rng();
    for _ in 0..200 {
        let r1 = rng.gen::<f64>();
        let r2 = rng.gen::<f64>();
        let _x = (2. * PI * r1).cos() * 2. * (r2 * (1. - r2)).sqrt();
        let _y = (2. * PI * r1).sin() * 2. * (r2 * (1. - r2)).sqrt();
        let z = 1. - r2;
        sum += z * z * z / (1. / (2. * PI));
    }

    println!("Pi/2 = {}", PI / 2.);
    println!("Estimate = {}", sum / n as f64);
}
