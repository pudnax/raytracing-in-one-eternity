use crate::{
    onb::{AxisBasis::*, Onb},
    vec3::Vec3,
    PdfObject, PI,
};

#[inline]
pub fn random_cosine_dir(rng: &mut dyn FnMut() -> f64) -> Vec3 {
    let r1 = rng();
    let r2 = rng();
    let z = (1. - r2).sqrt();

    let phi = 2. * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();
    Vec3(x, y, z)
}

pub enum Pdf {
    Cosine { uvw: Onb },
    Hittable(Inner),
    Mixture { p: Inner, q: Inner },
}

pub struct Inner {
    origin: Vec3,
    hittable: Box<dyn PdfObject>,
}

impl Inner {
    fn value(&self, origin: Vec3, direction: Vec3, rng: &mut dyn FnMut() -> f64) -> f64 {
        self.hittable.pdf_value(origin, direction, rng)
    }

    fn generate(&self, rng: &mut dyn FnMut(f64, f64) -> f64) -> Vec3 {
        self.hittable.random(self.origin, rng)
    }
}

impl Pdf {
    pub fn cosine(w: Vec3) -> Self {
        Pdf::Cosine {
            uvw: Onb::build_from_w(w),
        }
    }

    pub fn hittable(hittable: Box<dyn PdfObject>, origin: Vec3) -> Self {
        Pdf::Hittable(Inner { origin, hittable })
    }

    pub fn mixture(p: Inner, q: Inner) -> Self {
        Pdf::Mixture { p, q }
    }

    pub fn value(&self, direction: Vec3, rng: &mut dyn FnMut() -> f64) -> f64 {
        match self {
            Pdf::Cosine { uvw } => {
                let cosine = direction.into_unit().dot(uvw[W]);
                if cosine > 0.0 {
                    cosine / PI
                } else {
                    // FIXME: Or 0.0?
                    0.0
                }
            }
            Pdf::Hittable(Inner { origin, hittable }) => {
                hittable.pdf_value(*origin, direction, rng)
            }
            Pdf::Mixture { p, q } => {
                0.5 * p.value(p.origin, direction, rng) + 0.5 * q.value(q.origin, direction, rng)
            }
        }
    }

    pub fn generate(&self, rng: &mut dyn FnMut(f64, f64) -> f64) -> Vec3 {
        match self {
            Pdf::Cosine { uvw } => uvw.local(random_cosine_dir(&mut || rng(0., 1.))),
            Pdf::Hittable(Inner { origin, hittable }) => hittable.random(*origin, rng),
            Pdf::Mixture { p, q } => {
                if rng(0., 1.) < 0.5 {
                    p.generate(rng)
                } else {
                    q.generate(rng)
                }
            }
        }
    }
}
