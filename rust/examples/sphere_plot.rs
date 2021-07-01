use rand::prelude::*;
use std::f64::consts::PI;

fn main() {
    let mut rng = thread_rng();
    for _ in 0..200 {
        let r1 = rng.gen::<f64>();
        let r2 = rng.gen::<f64>();
        let x = (2. * PI * r1).cos() * 2. * (r2 * (1. - r2)).sqrt();
        let y = (2. * PI * r1).sin() * 2. * (r2 * (1. - r2)).sqrt();
        let z = 1. - 2. * r2;
        println!("{} {} {}\n", x, y, z);
    }
}
