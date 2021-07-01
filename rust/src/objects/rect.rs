use crate::{
    aabb::Aabb,
    objects::{HitRecord, Object, PdfObject},
    vec3::{
        Axis::{self, *},
        Vec3,
    },
    Material, Ray,
};
use std::ops::Range;

/// A rectangle orthogonal to one axis.
///
/// The rectangle is specified by the name of its orthogonal axis, and the
/// ranges in the other two axes. "Other two" is alphabetical, so for example,
/// if `orthogonal_to` is `StaticZ`, the other two are X and Y.
///
/// The axis is named at compile time from one of `StaticX`, `StaticY`, and
/// `StaticZ`. This gets us code customed to each case, without having separate
/// types for `RectXY`, `RectYZ`, and `RectXZ`.
#[derive(Debug, Clone)]
pub struct Rect<A: StaticAxis> {
    /// Axis normal to this rectangle.
    pub orthogonal_to: A,
    /// Range in alphabetically lower non-orthogonal axis.
    pub range0: Range<f64>,
    /// Range in alphabetically higher non-orthogonal axis.
    pub range1: Range<f64>,
    /// Position along the orthogonal axis.
    ///
    /// TODO(#5): replace with Translate?
    pub k: f64,
    /// Rectangle material.
    pub material: Material,
}

/// Trait implemented by static axis types for `Rect`.
pub trait StaticAxis: std::fmt::Debug + Send + Sync {
    const AXIS: Axis;
    const OTHER1: Axis;
    const OTHER2: Axis;
}

/// Compile-time (static) name for the X axis.
#[derive(Debug)]
pub struct StaticX;

impl StaticAxis for StaticX {
    const AXIS: Axis = X;
    const OTHER1: Axis = Y;
    const OTHER2: Axis = Z;
}

/// Compile-time (static) name for the Y axis.
#[derive(Debug)]
pub struct StaticY;

impl StaticAxis for StaticY {
    const AXIS: Axis = Y;
    const OTHER1: Axis = X;
    const OTHER2: Axis = Z;
}

/// Compile-time (static) name for the Z axis.
#[derive(Debug)]
pub struct StaticZ;

impl StaticAxis for StaticZ {
    const AXIS: Axis = Z;
    const OTHER1: Axis = X;
    const OTHER2: Axis = Y;
}

impl<A: StaticAxis> Object for Rect<A> {
    #[inline]
    #[allow(clippy::many_single_char_names)]
    fn hit<'o>(
        &'o self,
        ray: &Ray,
        t_range: Range<f64>,
        _rng: &mut dyn FnMut() -> f64,
    ) -> Option<HitRecord<'o>> {
        // The names x and y below are correct for orthogonal_to=Z.

        let t = (self.k - ray.origin[A::AXIS]) / ray.direction[A::AXIS];
        if t_range.start > t || t >= t_range.end {
            return None;
        }

        let x = ray.origin[A::OTHER1] + t * ray.direction[A::OTHER1];
        let y = ray.origin[A::OTHER2] + t * ray.direction[A::OTHER2];
        if x < self.range0.start
            || x >= self.range0.end
            || y < self.range1.start
            || y >= self.range1.end
        {
            return None;
        }

        let u = (x - self.range0.start) / (self.range0.end);
        let v = (y - self.range1.start) / (self.range1.end);

        let p = ray.point_at_parameter(t);
        let mut normal = Vec3::default();
        normal[A::AXIS] = 1.;
        Some(HitRecord {
            t,
            p,
            u,
            v,
            material: &self.material,
            normal,
        })
    }

    fn bounding_box(&self, _exposure: Range<f64>) -> Aabb {
        let mut min = Vec3::default();
        let mut max = Vec3::default();

        min[A::AXIS] = self.k - 0.0001;
        max[A::AXIS] = self.k + 0.0001;
        min[A::OTHER1] = self.range0.start;
        max[A::OTHER1] = self.range0.end;
        min[A::OTHER2] = self.range1.start;
        max[A::OTHER2] = self.range1.end;

        Aabb { min, max }
    }
}

impl<A: StaticAxis> PdfObject for Rect<A> {
    fn pdf_value(&self, origin: Vec3, v: Vec3, rng: &mut dyn FnMut() -> f64) -> f64 {
        let sample_ray = Ray::new(origin, v, 0.0);
        if let Some(hit) = self.hit(&sample_ray, 0.001..f64::MAX, &mut || rng()) {
            let area =
                (self.range1.end - self.range1.start) / (self.range0.end - self.range0.start);
            let distance_squared = hit.t * hit.t * v.dot(v);
            let cosine = v.dot(hit.normal).abs() / v.length();

            distance_squared / (cosine * area)
        } else {
            0.0
        }
    }
    fn random(&self, origin: Vec3, rng: &mut dyn FnMut(f64, f64) -> f64) -> Vec3 {
        let random_point = Vec3(
            rng(self.range1.start, self.range1.end),
            self.k,
            rng(self.range0.start, self.range0.end),
        );
        random_point - origin
    }
}
