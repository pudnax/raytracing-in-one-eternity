use crate::{ray::Ray, vec3::Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn merge(self, other: Aabb) -> Self {
        Aabb {
            min: self.min.zip_with(other.min, f64::min),
            max: self.max.zip_with(other.max, f64::max),
        }
    }

    pub fn hit(&self, ray: &Ray, t_range: std::ops::Range<f64>) -> bool {
        let inv_d = ray.direction.map(|x| 1. / x);
        let t0 = (self.min - ray.origin) * inv_d;
        let t1 = (self.max - ray.origin) * inv_d;
        let (t0, t1) = (
            inv_d.zip_with3(t0, t1, |i, a, b| if i < 0. { b } else { a }),
            inv_d.zip_with3(t0, t1, |i, a, b| if i < 0. { a } else { b }),
        );
        let start = t_range.start.max(t0.reduce(f64::max));
        let end = t_range.end.min(t1.reduce(f64::min));
        end > start
    }

    pub fn corners(&self) -> impl Iterator<Item = Vec3> + '_ {
        (0..2).flat_map(move |x| {
            (0..2).flat_map(move |y| {
                (0..2).map(move |z| {
                    Vec3(
                        if x == 0 { self.min.0 } else { self.max.0 },
                        if y == 0 { self.min.1 } else { self.max.1 },
                        if z == 0 { self.min.2 } else { self.max.2 },
                    )
                })
            })
        })
    }
}
