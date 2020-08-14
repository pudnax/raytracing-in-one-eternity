use rand::prelude::*;
use std::time::Instant;

use raytrace::{object::Object, scenes::*, *};

const USE_BVH: bool = true;

fn main() {
    const ASPECT_RATIO: f64 = 1.0 / 1.0;
    const NX: usize = 500;
    const NY: usize = (NX as f64 / ASPECT_RATIO) as usize;
    const NS: usize = 500;

    eprintln!(
        "Parallel casting {} x {} image using {}x oversampling.",
        NX, NY, NS
    );

    let mut rng = rand::rngs::SmallRng::seed_from_u64(0xDEADBEEF);
    rng.gen::<f64>();

    let (world, camera, exposure) = cornell_box_scene(NX, NY);
    // let (world, camera, exposure) = simple_light_scene(NX, NY, &mut rng);
    // let (world, camera, exposure) = volume_test(NX, NY);
    // let (world, camera, exposure) = book_final_scene(NX, NY, &mut rng);
    // let (world, camera, exposure) = scene_textured_sphere(NX, NY);

    let (image, time) = if USE_BVH {
        eprintln!("Generating bounding volume hierarchy.");
        let world = bvh::Bvh::new(world, exposure);
        eprintln!("Done.");
        let start = Instant::now();
        (par_cast(NX, NY, NS, &camera, world), start.elapsed())
    } else {
        eprintln!("Testing every ray against every object.");
        let world: &[Box<dyn Object>] = &world;
        let start = Instant::now();
        (par_cast(NX, NY, NS, &camera, world), start.elapsed())
    };

    eprintln!("Took {:?} wall time.", time);
    print_ppm(image);
}
