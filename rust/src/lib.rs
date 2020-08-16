mod aabb;
pub mod bvh;
pub mod camera;
mod image_texture;
pub mod material;
pub mod objects;
mod onb;
mod pdf;
mod perlin;
pub mod ray;
pub mod scenes;
pub mod texture;
pub mod vec3;

use rand::prelude::*;
use rayon::prelude::*;

pub use crate::{
    camera::Camera,
    material::Material,
    objects::{
        rect_prism, rotate_y, FlipNormals, HitRecord, Object, PdfObject, Rect, Sphere, StaticX,
        StaticY, StaticZ, Translate,
    },
    pdf::Pdf,
    ray::Ray,
    vec3::{Channel::*, *},
};

pub use std::f64::consts::{FRAC_PI_2, PI, TAU};

pub trait World: Send + Sync {
    fn hit_top<'a>(&'a self, ray: &Ray, rng: &mut impl Rng) -> Option<HitRecord<'a>>;
}

impl<'r, T: World + ?Sized> World for &'r T {
    fn hit_top<'a>(&'a self, ray: &Ray, rng: &mut impl Rng) -> Option<HitRecord<'a>> {
        (*self).hit_top(ray, rng)
    }
}

impl World for [Box<dyn Object>] {
    fn hit_top<'a>(&'a self, ray: &Ray, rng: &mut impl Rng) -> Option<HitRecord<'a>> {
        const NEAR: f64 = 0.001;

        let mut nearest = f64::MAX;
        let mut hit = None;

        for obj in self {
            if let Some(rec) = obj.hit(ray, NEAR..nearest, &mut || rng.gen()) {
                nearest = rec.t;
                hit = Some(rec);
            }
        }

        hit
    }
}

impl World for bvh::Bvh {
    fn hit_top<'a>(&'a self, ray: &Ray, rng: &mut impl Rng) -> Option<HitRecord<'a>> {
        self.hit(ray, 0.001..f64::MAX, &mut || rng.gen())
    }
}

/// Computes the pixel color along `ray` for the scene of objects `world`.
///
/// This is the actual ray-tracing routine.
pub fn ray_color(world: &impl World, mut ray: Ray, rng: &mut impl Rng) -> Vec3 {
    // Accumulates contribution of each surface we reach.
    let mut accum = Vec3::default();
    // Records the cumulative (product) attenuation of each surface we've
    // visited so far.
    let mut attenuation = Vec3::from(1.);

    let mut bounces = 0;

    // Iterate until one of the following conditions is reached:
    // 1. The ray escapes into space (i.e. no objects are hit).
    // 2. The ray reaches a surface that does not scatter.
    // 3. The ray bounces more than 50 times.
    while let Some(hit) = world.hit_top(&ray, rng) {
        // Record this hit's contribution, attenuated by the total attenuation
        // so far.
        accum = accum + attenuation * hit.material.emitted(hit.u, hit.v, hit.p, &hit);

        // Check whether the material scatters light, generating a new ray. In
        // practice this is true for everything but the emission-only
        // DiffuseLight type.
        //
        // TODO(#4): and also for frosted metal, which effectively makes frosted
        // metal an emitter. That can't be right.
        if let Some((scattered, albedo, _pdf)) = hit.material.scatter(&ray, &hit, rng) {
            // Redirect flight, accumulate the new attenuation value.
            attenuation = attenuation * albedo;
            ray = scattered;
        } else {
            // Locally absorbed; we're done.
            return accum;
        }

        // TODO(#6): Add `depth` as configureable argument as maximum
        // number of bounces
        if bounces == 50 {
            return accum;
        }

        bounces += 1;
    }

    // TODO: Add background color
    Vec3::default()
}

pub fn cornell_box() -> Vec<Box<dyn Object>> {
    fn diffuse_color(c: Vec3) -> Material {
        Material::Lambertian {
            albedo: texture::constant(c),
        }
    }

    let red = diffuse_color(Vec3(0.65, 0.05, 0.05));
    let white = diffuse_color(Vec3::from(0.73));
    let green = diffuse_color(Vec3(0.12, 0.45, 0.15));
    let light = Material::DiffuseLight {
        emission: texture::constant(Vec3::from(1.)),
        brightness: 15.,
    };
    vec![
        Box::new(Rect {
            orthogonal_to: StaticY,
            range0: 213. ..343.,
            range1: 227. ..332.,
            k: 554.,
            material: light,
        }),
        // floor
        Box::new(Rect {
            orthogonal_to: StaticY,
            range0: 0. ..555.,
            range1: 0. ..555.,
            k: 0.,
            material: white.clone(),
        }),
        // rear wall
        Box::new(FlipNormals(Rect {
            orthogonal_to: StaticZ,
            range0: 0. ..555.,
            range1: 0. ..555.,
            k: 555.,
            material: white.clone(),
        })),
        // ceiling
        Box::new(FlipNormals(Rect {
            orthogonal_to: StaticY,
            range0: 0. ..555.,
            range1: 0. ..555.,
            k: 555.,
            material: white.clone(),
        })),
        // right wall
        Box::new(Rect {
            orthogonal_to: StaticX,
            range0: 0. ..555.,
            range1: 0. ..555.,
            k: 0.,
            material: red,
        }),
        // left wall
        Box::new(FlipNormals(Rect {
            orthogonal_to: StaticX,
            range0: 0. ..555.,
            range1: 0. ..555.,
            k: 555.,
            material: green,
        })),
    ]
}

pub fn cornell_box_with_boxes() -> Vec<Box<dyn Object>> {
    fn diffuse_color(c: Vec3) -> Material {
        Material::Lambertian {
            albedo: texture::constant(c),
        }
    }

    let mut scene = cornell_box();
    let white = diffuse_color(Vec3::from(0.73));

    scene.push(Box::new(Translate {
        offset: Vec3(130., 0., 65.),
        object: rotate_y(
            -18.,
            rect_prism(Vec3(0., 0., 0.), Vec3(165., 165., 165.), white.clone()),
        ),
    }));
    scene.push(Box::new(Translate {
        offset: Vec3(265., 0., 295.),
        object: rotate_y(
            15.,
            rect_prism(Vec3(0., 0., 0.), Vec3(165., 330., 165.), white),
        ),
    }));
    scene
}

/*
pub fn simple_light() -> Vec<Object> {
    vec![
        Sphere {
            center: Vec3(0., -1000., 0.),
            radius: 1000.,
            material: Material::Lambertian {
                albedo: texture::perlin(4.),
            },
            motion: Vec3::default(),
        },
        Sphere {
            center: Vec3(0., 2., 0.),
            radius: 2.,
            material: Material::Lambertian {
                albedo: texture::perlin(4.),
            },
            motion: Vec3::default(),
        },
        Sphere {
            center: Vec3(0., 7., 0.),
            radius: 2.,
            material: Material::DiffuseLight {
                emission: texture::constant(Vec3(1., 1., 1.)),
                brightness: 4.,
            },
            motion: Vec3::default(),
        },
        Rect {
            orthogonal_to: StaticAxis::Z,
            range0: 3. ..5.,
            range1: 1. ..3.,
            k: -2.,
            material: Material::DiffuseLight {
                emission: texture::constant(Vec3(1., 1., 1.)),
                brightness: 4.,
            },
        },
    ]
}
*/

/*
pub fn random_scene(rng: &mut impl Rng) -> Vec<Object> {
    let mut world = vec![Sphere {
        center: Vec3(0., -1000., 0.),
        radius: 1000.,
        material: Material::Lambertian {
            albedo: texture::perlin(4.),
        },
        motion: Vec3::default(),
    }];

    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vec3(4., 0.2, 0.)).length() > 0.9 {
                let choose_mat = rng.gen::<f64>();

                let obj = if choose_mat < 0.8 {
                    Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Lambertian {
                            albedo: texture::constant(rng.gen::<Vec3>() * rng.gen::<Vec3>()),
                        },
                        motion: Vec3(0., rng.gen_range(0., 0.5), 0.),
                    }
                } else if choose_mat < 0.95 {
                    Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Metal {
                            albedo: 0.5 * (1. + rng.gen::<Vec3>()),
                            fuzz: 0.5 * rng.gen::<f64>(),
                        },
                        motion: Vec3::default(),
                    }
                } else {
                    Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Dielectric { ref_idx: 1.5 },
                        motion: Vec3::default(),
                    }
                };
                world.push(obj);
            }
        }
    }

    world.push(Sphere {
        center: Vec3(0., 1., 0.),
        radius: 1.0,
        material: Material::Dielectric { ref_idx: 1.5 },
        motion: Vec3::default(),
    });

    world.push(Sphere {
        center: Vec3(-4., 1., 0.),
        radius: 1.0,
        material: Material::Metal {
            albedo: Vec3(0.7, 0.6, 0.5),
            fuzz: 0.,
        },
        motion: Vec3::default(),
    });

    world.push(Sphere {
        center: Vec3(4., 1., 0.),
        radius: 1.0,
        material: Material::DiffuseLight {
            emission: texture::perlin(10.),
            brightness: 4.,
        },
        motion: Vec3::default(),
    });

    world
}
*/

pub struct Image(Vec<Vec<Vec3>>);

impl Image {
    pub fn par_compute(nx: usize, ny: usize, f: impl Fn(usize, usize) -> Vec3 + Sync) -> Image {
        Image(
            (0..ny)
                .into_par_iter()
                .rev()
                .map(|y| (0..nx).map(|x| f(x, y)).collect())
                .collect(),
        )
    }

    pub fn compute(nx: usize, ny: usize, mut f: impl FnMut(usize, usize) -> Vec3) -> Image {
        Image(
            (0..ny)
                .rev()
                .map(|y| (0..nx).map(|x| f(x, y)).collect())
                .collect(),
        )
    }
}

pub fn print_ppm(image: Image) {
    use std::io::Write;
    let mut writer = std::io::BufWriter::new(std::io::stdout());
    writer
        .write(format!("P3\n{} {}\n255\n", image.0[0].len(), image.0.len()).as_bytes())
        .unwrap();

    for scanline in image.0 {
        for col in scanline {
            let col = Vec3(col.0.sqrt(), col.1.sqrt(), col.2.sqrt());

            fn to_u8(x: f64) -> i32 {
                ((255.99 * x) as i32).max(0).min(255)
            }

            let ir = to_u8(col[R]);
            let ig = to_u8(col[G]);
            let ib = to_u8(col[B]);

            writer
                .write(format!("{} {} {}\n", ir, ig, ib).as_bytes())
                .unwrap();
        }
    }
}

pub fn par_cast(nx: usize, ny: usize, ns: usize, camera: &Camera, world: impl World) -> Image {
    Image::par_compute(nx, ny, |x, y| {
        let col: Vec3 = (0..ns)
            .map(|_| {
                let mut rng = thread_rng();
                let u = (x as f64 + rng.gen::<f64>()) / nx as f64;
                let v = (y as f64 + rng.gen::<f64>()) / ny as f64;
                let r = camera.get_ray(u, v, &mut rng);
                ray_color(&world, r, &mut rng)
            })
            .sum();
        col / ns as f64
    })
}

pub fn cast(
    nx: usize,
    ny: usize,
    ns: usize,
    camera: &Camera,
    world: impl World,
    rng: &mut impl Rng,
) -> Image {
    Image::compute(nx, ny, |x, y| {
        let col: Vec3 = (0..ns)
            .map(|_| {
                let u = (x as f64 + rng.gen::<f64>()) / nx as f64;
                let v = (y as f64 + rng.gen::<f64>()) / ny as f64;
                let r = camera.get_ray(u, v, rng);
                ray_color(&world, r, rng)
            })
            .sum();
        col / ns as f64
    })
}
