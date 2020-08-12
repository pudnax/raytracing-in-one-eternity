use std::sync::Arc;

use crate::{
    image_texture::{load_image, map_image},
    perlin,
    vec3::Vec3,
};

pub type Texture = Arc<dyn Fn(f64, f64, Vec3) -> Vec3 + Send + Sync>;

pub fn constant(color: Vec3) -> Texture {
    Arc::new(move |_, _, _| color)
}

pub fn checker(t0: Texture, t1: Texture) -> Texture {
    Arc::new(move |_, _, p| {
        let s = (10. * p).map(f64::sin).reduce(std::ops::Mul::mul);
        if s < 0. {
            t1(0., 0., p)
        } else {
            t0(0., 0., p)
        }
    })
}

pub fn marble_texture(scale: f64) -> Texture {
    Arc::new(move |_, _, p| {
        (0.5 * (1. + (scale * p.2 + 10. * perlin::turb(p, 7)).sin())) * Vec3(1., 1., 1.)
    })
}

pub fn perlin(scale: f64) -> Texture {
    Arc::new(move |_, _, p| Vec3::from(perlin::turb(scale * p, 7)))
}

pub fn matte(scale: f64) -> Texture {
    Arc::new(move |_, _, p| {
        Vec3::from(0.5 * (1. + f64::sin(scale * p.0 + 5. * perlin::turb(scale * p, 7))))
    })
}

pub fn image_texture<P: std::convert::AsRef<std::path::Path>>(
    filename: P,
) -> Result<Texture, Box<dyn std::error::Error>> {
    let (image, desc) = load_image(filename)?;
    Ok(Arc::new(move |u, v, _| map_image(u, v, image, desc)))
}
