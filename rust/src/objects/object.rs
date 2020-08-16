use std::ops::Range;

use crate::{aabb::Aabb, material::Material, ray::Ray, vec3::Vec3};

/// A description of a `Ray` hitting an `Object`. This stores information needed
/// for rendering later.
///
/// The `'m` lifetime refers to the `Material` of the `Object`, which we capture
/// by reference. Thus, a `Hit Record` cannot otlive the `Object` it refers to.
#[derive(Clone)]
pub struct HitRecord<'m> {
    /// Position along the ray. expressed in distance from the origin.
    pub t: f64,
    /// Position along the ray, as an actual point.
    pub p: Vec3,
    /// Horisontal texture coordinate in range [0, 1)
    pub u: f64,
    /// Vertical texture coordinate in range [0, 1)
    pub v: f64,
    /// Surface normal of the object at the position.
    pub normal: Vec3,
    /// Material of the object at the hit position.
    pub material: &'m Material,
}

pub trait PdfObject {
    fn pdf_value(&self, _origin: Vec3, _v: Vec3, _rng: &mut dyn FnMut() -> f64) -> f64 {
        0.0
    }
    fn random(&self, _origin: Vec3, _rng: &mut dyn FnMut(f64, f64) -> f64) -> Vec3 {
        Vec3(1.0, 0.0, 0.0)
    }
}

/// An object in a scene.
///
/// The primary purpose of an `Object` is to interact with rays of light using
/// the `hit` method.
pub trait Object: std::fmt::Debug + Sync + Send {
    /// Tests if `ray` intersects the object `self`, and if so, if that
    /// intersection occurs within `t_range` along the ray. (Recall that `Ray`
    /// is defined in terms of a `t` value that refers to points along the ray.)
    ///
    /// The `t_range` serves two purposes here. First, if the intersection
    /// occurs at *negative* `t`, the object is behind the photons instead of in
    /// front of them, and the intersection is an illusion. Second, while the
    /// upper end of `t_range` starts out as infinity, we adjust it down as we
    /// find objects along `ray`. Once we've found an object at position `t`, we
    /// can ignore any objects at positions greater than `t`.
    ///
    /// The `rng` is available for use by materials that have nondeterministic
    /// interaction with light, such as smoke and fog.
    fn hit<'o>(
        &'o self,
        ray: &Ray,
        t_range: Range<f64>,
        rng: &mut dyn FnMut() -> f64,
    ) -> Option<HitRecord<'o>>;

    /// Computes the bounding box for the object at the given range of times.
    /// This is called during scene setup, not rendering, and so it may be
    /// expensive.
    fn bounding_box(&self, exposure: Range<f64>) -> Aabb;
}

impl Object for Box<dyn Object> {
    fn hit<'o>(
        &'o self,
        ray: &Ray,
        t_range: Range<f64>,
        rng: &mut dyn FnMut() -> f64,
    ) -> Option<HitRecord<'o>> {
        (**self).hit(ray, t_range, rng)
    }
    fn bounding_box(&self, exposure: Range<f64>) -> Aabb {
        (**self).bounding_box(exposure)
    }
}
