use std::sync::Arc;

use crate::{perlin, vec3::Vec3};

pub type Texture = Arc<dyn Fn(Vec3) -> Vec3 + Send + Sync>;

pub fn constant(color: Vec3) -> Texture {
    Arc::new(move |_| color)
}

pub fn checker(t0: Texture, t1: Texture) -> Texture {
    Arc::new(move |p| {
        let s = (10. * p).map(f64::sin).reduce(std::ops::Mul::mul);
        if s < 0. {
            t1(p)
        } else {
            t0(p)
        }
    })
}

pub fn marble_texture(scale: f64) -> Texture {
    Arc::new(move |p| {
        (0.5 * (1. + (scale * p.2 + 10. * perlin::turb(p, 7)).sin())) * Vec3(1., 1., 1.)
    })
}

pub fn perlin(scale: f64) -> Texture {
    Arc::new(move |p| Vec3::from(perlin::turb(scale * p, 7)))
}

pub fn matte(scale: f64) -> Texture {
    Arc::new(move |p| {
        Vec3::from(0.5 * (1. + f64::sin(scale * p.0 + 5. * perlin::turb(scale * p, 7))))
    })
}
