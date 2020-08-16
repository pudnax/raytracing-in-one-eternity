use crate::{
    aabb::Aabb,
    objects::{HitRecord, Object},
    vec3::{Axis::*, Vec3},
    Material, Ray,
};
use std::ops::Range;

/// A sphere.
#[derive(Debug, Clone)]
pub struct Sphere {
    /// Position of center of the sphere
    pub center: Vec3,
    /// Radius of the sphere.
    pub radius: f64,
    /// Material of the sphere.
    pub material: Material,
}

fn get_sphere_uv(p: Vec3) -> (f64, f64) {
    use std::f64::consts::PI;
    let phi = f64::atan2(p[Z], p[X]);
    let theta = p[Y].asin();
    let u = 1. - (phi + PI) / (2. * PI);
    let v = (theta + PI / 2.) / PI;
    (u, v)
}

impl Object for Sphere {
    #[inline]
    fn hit<'o>(
        &'o self,
        ray: &Ray,
        t_range: Range<f64>,
        _rng: &mut dyn FnMut() -> f64,
    ) -> Option<HitRecord<'o>> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0. {
            for &t in &[
                (-b - discriminant.sqrt()) / a,
                (-b + discriminant.sqrt()) / a,
            ] {
                if t_range.start <= t && t < t_range.end {
                    let p = ray.point_at_parameter(t);
                    let (u, v) = get_sphere_uv((p - self.center) / self.radius);
                    return Some(HitRecord {
                        t,
                        p,
                        u,
                        v,
                        normal: (p - self.center) / self.radius,
                        material: &self.material,
                    });
                }
            }
        }
        None
    }

    fn bounding_box(&self, _exposure: Range<f64>) -> Aabb {
        Aabb {
            min: -Vec3::from(self.radius) + self.center,
            max: Vec3::from(self.radius) + self.center,
        }
    }
}
