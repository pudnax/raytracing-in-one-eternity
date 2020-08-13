use rand::prelude::*;
use std::time::Instant;

use raytrace::{object::Object, scenes::*, *};

const USE_BVH: bool = true;

fn main() {
    const NX: usize = 400;
    const NY: usize = 400;
    const NS: usize = 50;

    eprintln!(
        "Parallel casting {} x {} image using {}x oversampling.",
        NX, NY, NS
    );

    let mut rng = rand::rngs::SmallRng::seed_from_u64(0xDEADBEEF);

    //let (world, camera, exposure) = cornell_box_scene(NX, NY);
    //let (world, camera, exposure) = simple_light_scene(NX, NY, &mut rng);
    //let (world, camera, exposure) = volume_test(NX, NY);
    //let (world, camera, exposure) = book_final_scene(NX, NY, &mut rng);
    let (world, camera, exposure) = scene_textured_sphere(NX, NY);

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
