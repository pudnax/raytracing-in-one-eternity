use crate::{
    bvh,
    camera::Camera,
    material::{self, Material},
    objects::*,
    texture,
    vec3::Vec3,
};
use crate::{cornell_box, cornell_box_with_boxes};
use rand::prelude::*;
use std::ops::Range;

#[allow(unused)]
pub fn cornell_box_scene(nx: usize, ny: usize) -> (Vec<Box<dyn Object>>, Camera, Range<f64>) {
    let look_from = Vec3(278., 278., -800.);
    let look_at = Vec3(278., 278., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.0;
    let exposure = 0. ..1.;

    let camera = Camera::look(
        look_from,
        look_at,
        Vec3(0., 1., 0.),
        40.,
        nx as f64 / ny as f64,
        aperture,
        dist_to_focus,
        exposure.clone(),
    );

    (cornell_box_with_boxes(), camera, exposure)
}

#[allow(unused)]
pub fn motion_test(nx: usize, ny: usize) -> (Vec<Box<dyn Object>>, Camera, Range<f64>) {
    let look_from = Vec3(278., 278., -800.);
    let look_at = Vec3(278., 278., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.0;
    let exposure = 0. ..1.;

    let camera = Camera::look(
        look_from,
        look_at,
        Vec3(0., 1., 0.),
        40.,
        nx as f64 / ny as f64,
        aperture,
        dist_to_focus,
        exposure.clone(),
    );

    let mut scene = cornell_box();

    scene.push(Box::new(LinearMove {
        motion: Vec3(0., 100., 0.),
        object: Sphere {
            center: Vec3(278., 278., 278.),
            radius: 65.,
            material: Material::Lambertian {
                albedo: texture::constant(Vec3::from(0.73)),
            },
        },
    }));

    (scene, camera, exposure)
}

#[allow(unused)]
pub fn volume_test(nx: usize, ny: usize) -> (Vec<Box<dyn Object>>, Camera, Range<f64>) {
    let look_from = Vec3(278., 278., -800.);
    let look_at = Vec3(278., 278., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.0;
    let exposure = 0. ..1.;

    let camera = Camera::look(
        look_from,
        look_at,
        Vec3(0., 1., 0.),
        40.,
        nx as f64 / ny as f64,
        aperture,
        dist_to_focus,
        exposure.clone(),
    );

    let mut scene = cornell_box();

    scene.push(Box::new(ConstantMedium {
        boundary: Sphere {
            center: Vec3(278., 278., 278.),
            radius: 180.,
            // material does not matter here
            material: material::Material::Lambertian {
                albedo: texture::constant(Vec3::from(0.73)),
            },
        },
        density: 0.01,
        material: material::Material::Isotropic {
            albedo: texture::constant(Vec3(0.2, 0.2, 1.0)),
        },
    }));

    (scene, camera, exposure)
}

#[allow(unused)]
pub fn simple_light_scene(
    nx: usize,
    ny: usize,
    rng: &mut impl Rng,
) -> (Vec<Box<dyn Object>>, Camera, Range<f64>) {
    let look_from = Vec3(278., 278., -800.);
    let look_at = Vec3(278., 278., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.0;
    let exposure = 0. ..1.;

    let camera = Camera::look(
        look_from,
        look_at,
        Vec3(0., 1., 0.),
        40.,
        nx as f64 / ny as f64,
        aperture,
        dist_to_focus,
        exposure.clone(),
    );

    use material::Material;

    let mut world = cornell_box();

    const SPHERES: usize = 1000;
    for _ in 0..SPHERES {
        let pos = 277. + 257. * rng.gen::<Vec3>();
        world.push(Box::new(Sphere {
            center: pos,
            radius: 20.,
            material: Material::Lambertian {
                albedo: texture::constant(Vec3::from(0.3)),
            },
        }));
    }

    world.push(Box::new(FlipNormals(Sphere {
        center: Vec3::default(),
        radius: 1000.,
        material: Material::DiffuseLight {
            emission: texture::constant(Vec3::from(0.1)),
            brightness: 1.,
        },
    })));

    (world, camera, exposure)
}

#[allow(unused)]
pub fn scene_textured_sphere(nx: usize, ny: usize) -> (Vec<Box<dyn Object>>, Camera, Range<f64>) {
    let r = 675.;
    let theta = 90. * std::f64::consts::PI / 180.;
    let phi = 1. * std::f64::consts::PI / 180.;
    let x = r * theta.sin() * phi.cos();
    let y = r * theta.sin() * phi.sin();
    let z = r * theta.cos();

    let look_from = Vec3(x, y, z);
    let look_at = Vec3(0., 0., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.0;
    let exposure = 0. ..1.;

    let camera = Camera::look(
        look_from,
        look_at,
        Vec3(0., 1., 0.),
        40.,
        nx as f64 / ny as f64,
        aperture,
        dist_to_focus,
        exposure.clone(),
    );

    let mut world: Vec<Box<dyn Object>> = vec![];

    // Textured sphere.
    world.push(Box::new(rotate_y(
        23.,
        Sphere {
            center: look_at,
            radius: 60.,
            material: Material::DiffuseLight {
                emission: texture::image_texture("assets/jasmine.png").unwrap(),
                brightness: 1.,
            },
        },
    )));

    // let ground = Material::Lambertian {
    //     albedo: texture::checker(
    //         texture::constant(Vec3(255., 253., 237.).map(|x| x / 255.)),
    //         texture::constant(Vec3(237., 191., 163.).map(|x| x / 255.)),
    //         0.02,
    //     ),
    // };
    // world.push(Box::new(Sphere {
    //     center: Vec3(0., -10000., 0.),
    //     radius: 10000.,
    //     material: ground,
    // }));
    //
    world.push(Box::new(rotate_y(
        190.,
        FlipNormals(Sphere {
            center: Vec3(0., 0., 0.),
            radius: 100000.,
            material: Material::DiffuseLight {
                // emission: texture::constant(Vec3(1., 1., 1.)),
                // emission: texture::image_texture("assets/earthmap.jpg").unwrap(),
                emission: texture::matte(0.000091),
                // emission: texture::perlin(0.00002),
                brightness: 2.,
            },
        }),
    )));

    // Make light.
    world.push(Box::new(Rect {
        orthogonal_to: StaticX,
        range0: -123. ..423.,
        range1: -112. ..412.,
        k: 950.,
        material: Material::DiffuseLight {
            // emission: texture::constant(Vec3(227., 193., 111.).map(|x| x / 255.)),
            emission: texture::constant(Vec3::from(0.5)),
            brightness: 20.,
        },
    }));

    (world, camera, exposure)
}

pub fn book_final_scene(
    nx: usize,
    ny: usize,
    rng: &mut impl Rng,
) -> (Vec<Box<dyn Object>>, Camera, Range<f64>) {
    let look_from = Vec3(478., 278., -600.);
    let look_at = Vec3(278., 278., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.0;
    let exposure = 0. ..1.;

    let camera = Camera::look(
        look_from,
        look_at,
        Vec3(0., 1., 0.),
        40.,
        nx as f64 / ny as f64,
        aperture,
        dist_to_focus,
        exposure.clone(),
    );

    let ground = Material::Lambertian {
        albedo: texture::constant(Vec3(0.48, 0.83, 0.53)),
    };

    let mut world: Vec<Box<dyn Object>> = vec![];

    // Make random floor.
    world.push({
        let mut boxes: Vec<Box<dyn Object>> = vec![];
        for i in 0..20 {
            for j in 0..20 {
                const W: f64 = 100.;
                let c0 = Vec3(-1000. + i as f64 * W, 0., -1000. + j as f64 * W);
                let c1 = c0 + Vec3(W, 100. * (rng.gen::<f64>() + 0.01), W);
                boxes.push(Box::new(rect_prism(c0, c1, ground.clone())));
            }
        }
        Box::new(bvh::Bvh::new(boxes, exposure.clone()))
    });

    // Make light.
    world.push(Box::new(Rect {
        orthogonal_to: StaticY,
        range0: 123. ..423.,
        range1: 147. ..412.,
        k: 554.,
        material: Material::DiffuseLight {
            emission: texture::constant(Vec3::from(1.)),
            brightness: 7.,
        },
    }));

    // Brown blurry sphere.
    world.push(Box::new(LinearMove {
        motion: Vec3(30., 0., 0.),
        object: Sphere {
            center: Vec3(400., 400., 200.),
            radius: 50.,
            material: Material::Lambertian {
                albedo: texture::constant(Vec3(0.7, 0.3, 0.1)),
            },
        },
    }));

    let glass = Material::Dielectric { ref_idx: 1.5 };

    // Glass sphere.
    world.push(Box::new(Sphere {
        center: Vec3(260., 150., 45.),
        radius: 50.,
        material: glass.clone(),
    }));

    // Textured sphere.
    world.push(Box::new(Sphere {
        center: Vec3(400., 200., 400.),
        radius: 100.,
        material: Material::Lambertian {
            albedo: texture::image_texture("assets/earthmap.jpg").unwrap(),
        },
    }));

    // Silvery sphere.
    world.push(Box::new(Sphere {
        center: Vec3(0., 150., 145.),
        radius: 50.,
        material: Material::Metal {
            albedo: Vec3(0.8, 0.8, 0.9),
            fuzz: 1.,
        },
    }));

    // Blue glass sphere.
    let boundary = Sphere {
        center: Vec3(360., 150., 145.),
        radius: 70.,
        material: glass.clone(),
    };
    world.push(Box::new(boundary.clone()));
    world.push(Box::new(ConstantMedium {
        boundary,
        density: 0.2,
        material: Material::Isotropic {
            albedo: texture::constant(Vec3(0.2, 0.4, 0.9)),
        },
    }));

    // Fog.
    world.push(Box::new(ConstantMedium {
        boundary: Sphere {
            center: Vec3::default(),
            radius: 5000.,
            material: glass.clone(), // doesn't matter
        },
        density: 0.0001,
        material: Material::Isotropic {
            albedo: texture::constant(Vec3::from(1.)),
        },
    }));

    // Perlin marbled sphere.
    world.push(Box::new(Sphere {
        center: Vec3(220., 280., 300.),
        radius: 80.,
        material: Material::Lambertian {
            albedo: texture::perlin(0.05),
        },
    }));

    // Cube made of random spheres.
    world.push(Box::new({
        const SPHERES: usize = 1000;
        let white = Material::Lambertian {
            albedo: texture::constant(Vec3::from(0.73)),
        };
        let spheres = (0..SPHERES)
            .map(|_| {
                Box::new(Sphere {
                    center: 165. * rng.gen::<Vec3>(),
                    radius: 10.,
                    material: white.clone(),
                }) as Box<dyn Object>
            })
            .collect();
        let bvh = bvh::Bvh::new(spheres, exposure.clone());
        Translate {
            offset: Vec3(-100., 270., 395.),
            object: rotate_y(15., bvh),
        }
    }));

    (world, camera, exposure)
}
