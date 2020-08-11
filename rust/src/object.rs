use std::ops::Range;

use crate::{
    aabb::Aabb,
    ray::Ray,
    vec3::{
        Axis::{self, *},
        Vec3,
    },
};

/// A description of a `Ray` hitting an `Object`. This stores information needed
/// for rendering later.
///
/// The `'m` lifetime refers to the `Material` of the `Object`, which we capture
/// by reference. Thus, a `Hit Record` cannot otlive the `Object` it refers to.
#[derive(Clone)]
pub struct HitRecor<'m> {
    /// Position along the ray. expressed in distance from the origin.
    pub t: f32,
    /// Position along the ray, as an actual point.
    pub p: Vec3,
    /// Surface normal of the object at the position.
    pub normal: Vec3,
    /// Material of the object at the hit position.
    pub material: &'m Material,
}
