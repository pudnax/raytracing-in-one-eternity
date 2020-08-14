use crate::vec3::Vec3;
use image::flat::SampleLayout;
use std::convert::AsRef;

pub fn load_image<P: AsRef<std::path::Path>>(
    path: P,
    // TODO: Remove boxed error
) -> Result<(Vec<u8>, SampleLayout), Box<dyn std::error::Error>> {
    let image = image::open(path)?.into_rgb();
    let image_description = image.sample_layout();

    Ok((image.into_raw(), image_description))
}

pub fn map_image(u: f64, v: f64, image: &[u8], img_desc: SampleLayout) -> Vec3 {
    let channels = img_desc.channels as usize;
    let width = img_desc.width as usize;
    let height = img_desc.height as usize;

    let u = u.max(0.).min(1.);
    let v = 1. - v.max(0.).min(1.);

    let i = (u * width as f64) as usize;
    let j = (v * height as f64) as usize;

    let color_scale = 1.0 / 255.0;
    let rel_idx = j * width * channels + i * channels;
    let pixel = &image[rel_idx..rel_idx + 3];

    Vec3(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64).map(|x| x * color_scale)
}
