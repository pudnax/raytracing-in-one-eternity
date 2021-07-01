use rand::prelude::*;

use crate::{
    ray::Ray,
    vec3::{Axis::*, Vec3},
};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
    exposure: std::ops::Range<f64>,
}

impl Camera {
    pub fn look(
        look_from: Vec3,
        look_at: Vec3,
        up: Vec3,
        fov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
        exposure: std::ops::Range<f64>,
    ) -> Self {
        let lens_radius = aperture / 2.;
        let theta = fov * std::f64::consts::PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;
        let origin = look_from;
        let w = (look_from - look_at).into_unit();
        let u = up.cross(&w).into_unit();
        let v = w.cross(&u);
        let lower_left_corner =
            origin - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;
        let horizontal = 2. * half_width * focus_dist * u;
        let vertical = 2. * half_height * focus_dist * v;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
            exposure,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64, rng: &mut impl Rng) -> Ray {
        let rd = self.lens_radius * Vec3::in_unit_disc(rng);
        let offset = rd[X] * self.u + rd[Y] * self.v;
        let time = rng.gen_range(self.exposure.start..self.exposure.end);
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
            time,
        }
    }
}
