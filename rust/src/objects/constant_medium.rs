use crate::{
    aabb::Aabb,
    objects::{HitRecord, Object},
    vec3::Vec3,
    Material, Ray,
};
use std::ops::Range;

/// A medium of constant density that scatters light internally, such as
/// (greatly simplified) smoke or fog.
#[derive(Debug, Clone)]
pub struct ConstantMedium<O> {
    /// Outer boundary of the medium, expressed as another object.
    pub boundary: O,
    /// Density of the medium -- how likely is a scattering event per unit
    /// attenuated
    pub density: f64,
    /// Material that controls scattering behavior.
    pub material: Material,
}

impl<O: Object> Object for ConstantMedium<O> {
    #[inline]
    fn hit<'o>(
        &'o self,
        ray: &Ray,
        t_range: Range<f64>,
        rng: &mut dyn FnMut() -> f64,
    ) -> Option<HitRecord<'o>> {
        if let Some(mut hit1) = self.boundary.hit(ray, f64::MIN..f64::MAX, rng) {
            if let Some(mut hit2) = self.boundary.hit(ray, hit1.t + 0.0001..f64::MAX, rng) {
                hit1.t = hit1.t.max(t_range.start);
                hit2.t = hit2.t.min(t_range.end);
                if hit1.t >= hit2.t {
                    return None;
                }

                debug_assert!(hit1.t >= 0.);

                let distance_inside = (hit2.t - hit1.t) * ray.direction.length();
                let hit_distance = -(1. / self.density) * rng().ln();
                if hit_distance < distance_inside {
                    let t = hit1.t + hit_distance / ray.direction.length();
                    return Some(HitRecord {
                        t,
                        p: ray.point_at_parameter(t),
                        u: 0.,
                        v: 0.,
                        normal: Vec3(1., 0., 0.),
                        material: &self.material,
                    });
                }
            }
        }
        None
    }

    fn bounding_box(&self, exposure: Range<f64>) -> Aabb {
        self.boundary.bounding_box(exposure)
    }
}
