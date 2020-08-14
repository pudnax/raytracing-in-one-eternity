use crate::Vec3;

/// Orthonormal Basis. Each axis is orthogonal to each other, which
/// means cross prduct equals to 0([u x v] == 0).
#[derive(Debug)]
pub struct Onb(pub Vec3, pub Vec3, pub Vec3);

impl Onb {
    pub fn build_from_w(normal: Vec3) -> Onb {
        let w = normal.into_unit();
        let a = if w.0 > 0.9 {
            Vec3(0., 1., 0.)
        } else {
            Vec3(1., 0., 0.)
        };
        let v = w.cross(&a).into_unit();
        let u = w.cross(&v);
        Onb(u, v, w)
    }

    /// Returns a vector in a current basis.
    pub fn local(&self, a: Vec3) -> Vec3 {
        a.0 * self[U] + a.1 * self[V] + a.2 * self[W]
    }
}

/// Names for Onb lanes when used as a coordinate, exactly the same like
/// with vector.
///
/// `Onb` has an `Index` impl for `Axis`, so you can use `Axis` values to
/// select components from a `Vec3`:
#[derive(Copy, Clone, Debug)]
pub enum AxisBasis {
    U,
    V,
    W,
}

use AxisBasis::*;

impl ::std::ops::Index<AxisBasis> for Onb {
    type Output = Vec3;

    #[inline]
    fn index(&self, idx: AxisBasis) -> &Self::Output {
        match idx {
            U => &self.0,
            V => &self.1,
            W => &self.2,
        }
    }
}

impl ::std::ops::IndexMut<AxisBasis> for Onb {
    #[inline]
    fn index_mut(&mut self, idx: AxisBasis) -> &mut Self::Output {
        match idx {
            U => &mut self.0,
            V => &mut self.1,
            W => &mut self.2,
        }
    }
}
