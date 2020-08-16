use crate::{
    aabb::Aabb,
    objects::{HitRecord, Object},
    vec3::Vec3,
    Ray,
};
use std::ops::Range;

/// The same geometry as the contained `O`, but with the normal vectors
/// inverted.
#[derive(Debug, Clone)]
pub struct FlipNormals<O>(pub O);

impl<O: Object> Object for FlipNormals<O> {
    #[inline]
    fn hit<'o>(
        &'o self,
        ray: &Ray,
        t_range: Range<f64>,
        rng: &mut dyn FnMut() -> f64,
    ) -> Option<HitRecord<'o>> {
        self.0.hit(ray, t_range, rng).map(|h| HitRecord {
            normal: -h.normal,
            ..h
        })
    }

    fn bounding_box(&self, exposure: Range<f64>) -> Aabb {
        self.0.bounding_box(exposure)
    }
}

/// The same geometry as `O`, but translated by `offset` from the origin.
#[derive(Debug, Clone)]
pub struct Translate<O> {
    pub offset: Vec3,
    pub object: O,
}

impl<T: Object> Object for Translate<T> {
    #[inline]
    fn hit<'o>(
        &'o self,
        ray: &Ray,
        t_range: Range<f64>,
        rng: &mut dyn FnMut() -> f64,
    ) -> Option<HitRecord<'o>> {
        let t_ray = Ray {
            origin: ray.origin - self.offset,
            ..*ray
        };
        self.object.hit(&t_ray, t_range, rng).map(|hit| HitRecord {
            p: hit.p + self.offset,
            ..hit
        })
    }

    fn bounding_box(&self, exposure: Range<f64>) -> Aabb {
        let b = self.object.bounding_box(exposure);
        Aabb {
            min: b.min + self.offset,
            max: b.max + self.offset,
        }
    }
}

/// The same geometry as `O`, but scaled by `factor` on each axis.
#[derive(Debug, Clone)]
pub struct Scale<O> {
    pub factor: Vec3,
    pub object: O,
}

impl<T: Object> Object for Scale<T> {
    #[inline]
    fn hit<'o>(
        &'o self,
        ray: &Ray,
        t_range: Range<f64>,
        rng: &mut dyn FnMut() -> f64,
    ) -> Option<HitRecord<'o>> {
        let t_ray = Ray {
            origin: ray.origin / self.factor,
            direction: ray.direction / self.factor,
            ..*ray
        };
        self.object.hit(&t_ray, t_range, rng).map(|hit| HitRecord {
            p: hit.p * self.factor,
            normal: hit.normal / self.factor,
            ..hit
        })
    }

    fn bounding_box(&self, exposure: Range<f64>) -> Aabb {
        let b = self.object.bounding_box(exposure);
        Aabb {
            min: b.min * self.factor,
            max: b.max * self.factor,
        }
    }
}

/// The same geometry as `O`, but rotated around the Y axis.
///
/// Use the `rotate_y` function to obtain one of these that's been filled out
/// correctly.
#[derive(Debug, Clone)]
pub struct RotateY<O> {
    pub object: O,
    sin_theta: f64,
    cos_theta: f64,
}

impl<T: Object> Object for RotateY<T> {
    #[inline]
    fn hit<'o>(
        &'o self,
        ray: &Ray,
        t_range: Range<f64>,
        rng: &mut dyn FnMut() -> f64,
    ) -> Option<HitRecord<'o>> {
        fn rot(p: Vec3, sin_theta: f64, cos_theta: f64) -> Vec3 {
            Vec3(
                p.dot(Vec3(cos_theta, 0., sin_theta)),
                p.dot(Vec3(0., 1., 0.)),
                p.dot(Vec3(-sin_theta, 0., cos_theta)),
            )
        }

        let rot_ray = Ray {
            origin: rot(ray.origin, -self.sin_theta, self.cos_theta),
            direction: rot(ray.direction, -self.sin_theta, self.cos_theta),
            ..*ray
        };

        self.object
            .hit(&rot_ray, t_range, rng)
            .map(|hit| HitRecord {
                p: rot(hit.p, self.sin_theta, self.cos_theta),
                normal: rot(hit.normal, self.sin_theta, self.cos_theta),
                ..hit
            })
    }

    fn bounding_box(&self, exposure: Range<f64>) -> Aabb {
        fn rot(p: Vec3, sin_theta: f64, cos_theta: f64) -> Vec3 {
            Vec3(
                p.dot(Vec3(cos_theta, 0., sin_theta)),
                p.dot(Vec3(0., 1., 0.)),
                p.dot(Vec3(-sin_theta, 0., cos_theta)),
            )
        }

        let (min, max) = self.object.bounding_box(exposure).corners().fold(
            (Vec3::from(std::f64::MAX), Vec3::from(std::f64::MIN)),
            |(min, max), c| {
                let rot_c = rot(c, self.sin_theta, self.cos_theta);
                (min.zip_with(rot_c, f64::min), max.zip_with(rot_c, f64::max))
            },
        );
        Aabb { min, max }
    }
}

/// Combines both `T` and `S` into one `Object`.
#[derive(Debug, Clone)]
pub struct And<T, S>(pub T, pub S);

impl<T: Object, S: Object> Object for And<T, S> {
    fn hit<'o>(
        &'o self,
        ray: &Ray,
        mut t_range: Range<f64>,
        rng: &mut dyn FnMut() -> f64,
    ) -> Option<HitRecord<'o>> {
        let hit0 = self.0.hit(ray, t_range.clone(), rng);
        if let Some(h) = &hit0 {
            t_range.end = h.t
        }

        let hit1 = self.1.hit(ray, t_range, rng);
        hit1.or(hit0)
    }

    fn bounding_box(&self, exposure: Range<f64>) -> Aabb {
        self.0
            .bounding_box(exposure.clone())
            .merge(self.1.bounding_box(exposure))
    }
}

/// Returns a version of `object` that has been rotated `degrees` around the Y
/// axis.
pub fn rotate_y<O: Object>(degrees: f64, object: O) -> RotateY<O> {
    let radians = degrees * std::f64::consts::PI / 180.;
    RotateY {
        object,
        sin_theta: radians.sin(),
        cos_theta: radians.cos(),
    }
}

/// Imposes a motion vector on an object, causing motion blur proportional to
/// the length of the motion vector times the length of the exposure.
#[derive(Debug, Clone)]
pub struct LinearMove<O> {
    /// The object being moved.
    pub object: O,
    /// Its motion per unit time.
    pub motion: Vec3,
}

impl<O: Object> Object for LinearMove<O> {
    #[inline]
    fn hit<'o>(
        &'o self,
        ray: &Ray,
        t_range: Range<f64>,
        rng: &mut dyn FnMut() -> f64,
    ) -> Option<HitRecord<'o>> {
        self.object.hit(
            &Ray {
                origin: ray.origin - ray.time * self.motion,
                ..*ray
            },
            t_range,
            rng,
        )
    }

    fn bounding_box(&self, exposure: Range<f64>) -> Aabb {
        let bb = self.object.bounding_box(exposure.clone());

        let bb_start = Aabb {
            min: bb.min + exposure.start * self.motion,
            max: bb.max + exposure.start * self.motion,
        };
        let bb_end = Aabb {
            min: bb.min + exposure.end * self.motion,
            max: bb.max + exposure.end * self.motion,
        };

        bb_start.merge(bb_end)
    }
}
