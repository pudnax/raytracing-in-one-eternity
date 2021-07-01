use rand::prelude::*;
use raytrace::vec3::Vec3;

fn main() {
    let n = 1_000_000;
    let mut sum = 0.0;
    let mut rng = thread_rng();
    for _ in 0..n {
        let d = rng.gen::<Vec3>();
        let cosine_squared = d.2 * d.2;
        sum += cosine_squared / pdf(d);
    }

    println!("I = {}", sum / n as f64);
}

fn pdf(_p: Vec3) -> f64 {
    1. / (4. * std::f64::consts::PI)
}
