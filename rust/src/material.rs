use rand::prelude::*;

use crate::{
    objects::HitRecord,
    onb::{AxisBasis::*, Onb},
    pdf,
    ray::Ray,
    texture::Texture,
    vec3::{reflect, refract, Vec3},
    PI,
};

/// Material options for a rendered object.
#[derive(Clone)]
pub enum Material {
    /// An opaque material with a matte surface, where lighting is calculated
    /// using [Lambertian reflectance][lambert].
    ///
    /// [lambert]: https://en.wikipedia.org/wiki/Lambertian_reflectance
    Lambertian { albedo: Texture },
    /// A reflective material that looks like polished or frosted metal.
    Metal {
        /// The amount of light energy reflected in each color component, so
        /// `Vec3(1., 1., 1.)` is a white surface, and `Vec3(0., 0., 0.)` is
        /// totally black.
        albedo: Vec3,
        /// The amount of randomness introduced into reflected rays. A `fuzz` of
        /// 0 makes the surface look polished and mirror-smooth, while a `fuzz`
        /// of 1 produces a frosted, almost matte surface.
        fuzz: f64,
    },
    /// A transparent refractive material like glass or water.
    Dielectric {
        /// [Refractive index][ref-idx] of the material, which determines how
        /// much light is bent when traveling into or out of an object.
        ///
        /// [ref-idx]: https://en.wikipedia.org/wiki/Refractive_index
        ref_idx: f64,
    },
    /// Diffuse light.
    DiffuseLight { emission: Texture, brightness: f64 },
    /// Isotropic scattering.
    Isotropic { albedo: Texture },
}

impl Material {
    /// Performs surface scattering from a material.
    ///
    /// When light traveling along `ray` reaches a surface made out of this
    /// material (intersection described by `hit`), some of it will be absorbed,
    /// and the rest will either be reflected or refracted. If 100% of the light
    /// is absorbed, `scatter` returns `None`; otherwise, it returns a new `Ray`
    /// giving the reflected/refracted direction of the light, and a `Vec3` with
    /// the amount of energy reflected/refracted in each of red, green, and
    /// blue.
    ///
    /// (In reality, light would be *both* reflected and refracted, but we
    /// choose one or the other randomly and use over-sampling to produce a
    /// blend.)
    // TODO: PDF implemented only for Lambertian material.
    pub fn scatter(
        &self,
        ray: &Ray,
        hit: &HitRecord,
        rng: &mut impl Rng,
    ) -> Option<(Ray, Vec3, f64)> {
        match self {
            Material::Lambertian { albedo } => {
                let uvw = Onb::build_from_w(hit.normal);
                let direction = uvw.local(pdf::random_cosine_dir(&mut || rng.gen()));
                let scattered = Ray {
                    origin: hit.p,
                    direction: direction.into_unit(),
                    time: ray.time,
                };
                let pdf = uvw[W].dot(scattered.direction) / PI;
                Some((scattered, albedo(hit.u, hit.v, hit.p), pdf))
            }
            Material::Metal { albedo, fuzz } => {
                let scattered = Ray {
                    origin: hit.p,
                    direction: reflect(ray.direction.into_unit(), hit.normal)
                        + *fuzz * Vec3::in_unit_sphere(rng),
                    ..*ray
                };
                if scattered.direction.dot(hit.normal) > 0. {
                    Some((scattered, *albedo, 0.))
                } else {
                    // TODO(#3): this is in the original, but has the odd effect of
                    // making metal an emitter.
                    None
                }
            }
            Material::Dielectric { ref_idx } => {
                let (outward_normal, ni_over_nt, cosine) = if ray.direction.dot(hit.normal) > 0. {
                    (
                        -hit.normal,
                        *ref_idx,
                        *ref_idx * ray.direction.dot(hit.normal) / ray.direction.length(),
                    )
                } else {
                    (
                        hit.normal,
                        1.0 / *ref_idx,
                        -ray.direction.dot(hit.normal) / ray.direction.length(),
                    )
                };

                let direction = refract(ray.direction, outward_normal, ni_over_nt)
                    .filter(|_| rng.gen::<f64>() >= schlick(cosine, *ref_idx))
                    .unwrap_or_else(|| reflect(ray.direction, hit.normal));

                let attenuation = Vec3::from(1.);
                let ray = Ray {
                    origin: hit.p,
                    direction,
                    time: ray.time,
                };
                Some((ray, attenuation, 0.))
            }
            Material::DiffuseLight { .. } => None,
            Material::Isotropic { albedo } => Some((
                Ray {
                    origin: hit.p,
                    direction: Vec3::in_unit_sphere(rng),
                    ..*ray
                },
                albedo(hit.u, hit.v, hit.p),
                0.,
            )),
        }
    }

    pub fn scattering_pdf(&self, _ray: &Ray, hit: &HitRecord, scattered: &Ray) -> f64 {
        match self {
            Material::Lambertian { .. } => {
                let cosine = hit.normal.dot(scattered.direction.into_unit());
                if cosine < 0. {
                    return 0.;
                } else {
                    return cosine / PI;
                }
            }
            _ => 0.,
        }
    }

    /// Perfoms a light emitting from a light sources. The all non-emitting
    /// materials return black colour by default.
    // TODO: Remove reference to `HitRecord` which is self.
    pub fn emitted(&self, u: f64, v: f64, p: Vec3, hit: &HitRecord) -> Vec3 {
        match self {
            Material::DiffuseLight {
                emission,
                brightness,
            } => {
                if p.into_unit().dot(hit.normal) > 0. {
                    *brightness * emission(u, v, p)
                } else {
                    Vec3::from(0.)
                }
            }
            _ => Vec3::default(),
        }
    }
}

impl std::fmt::Debug for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("...")
    }
}

/// [Schlick's approximation][schlick] for computing reflection vs. refraction
/// at a material surface.
///
/// [schlick]: https://en.wikipedia.org/wiki/Schlick%27s_approximation
#[inline]
fn schlick(cos: f64, ref_idx: f64) -> f64 {
    let r0 = (1. - ref_idx) / (1. + ref_idx);
    let r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cos).powi(5)
}
