use rand::prelude::*;

use crate::vec3::Vec3;

const SIZE: u8 = 255;

fn generate_perm(rng: &mut impl Rng) -> Vec<u8> {
    let mut p: Vec<_> = (0..=SIZE).collect();

    for i in (1..=SIZE as usize).rev() {
        p.swap(i, rng.gen_range(0..i));
    }
    p
}

fn generate_vecs(rng: &mut impl Rng) -> Vec<Vec3> {
    (0..=SIZE).map(|_| Vec3::in_unit_sphere(rng)).collect()
}

lazy_static::lazy_static! {
    pub static ref VECS: Vec<Vec3> = generate_vecs(&mut thread_rng());
    pub static ref PERM_X: Vec<u8> = generate_perm(&mut thread_rng());
    pub static ref PERM_Y: Vec<u8> = generate_perm(&mut thread_rng());
    pub static ref PERM_Z: Vec<u8> = generate_perm(&mut thread_rng());
}

fn trilinear_interp<'a>(corner_iter: impl Iterator<Item = &'a Vec3>, uvw: Vec3) -> f64 {
    let uvw3 = uvw * uvw * (Vec3::from(3.) - 2. * uvw);
    let uvw3_inv = Vec3::from(1.) - uvw3;
    corner_iter.enumerate().fold(0., |accum, (idx, corner)| {
        let (i, j, k) = (idx & 1, (idx & 2) / 2, (idx & 4) / 4);
        let ijk = Vec3(i as f64, j as f64, k as f64);
        let weight = corner.dot(uvw - ijk);
        let ijk_inv = Vec3::from(1.) - ijk;
        accum + (ijk + uvw3 + ijk_inv * uvw3_inv).reduce(std::ops::Mul::mul) * weight
    })
}

#[allow(dead_code)]
fn trilinear_interp_array(corners: &[[[Vec3; 2]; 2]; 2], uvw: Vec3) -> f64 {
    trilinear_interp(corners.iter().flatten().flatten(), uvw)
}

pub fn noise(p: Vec3) -> f64 {
    let ijk = p.map(f64::floor);
    let uvw = p - ijk;
    let mut corners = [[[Vec3::default(); 2]; 2]; 2];
    for (idx, corner) in corners.iter_mut().flatten().flatten().enumerate() {
        let (di, dj, dk) = (idx & 1, (idx & 2) / 2, (idx & 4) / 4);
        let ix = PERM_X[((ijk.0 as i32 + di as i32) & 255) as usize];
        let iy = PERM_Y[((ijk.1 as i32 + dj as i32) & 255) as usize];
        let iz = PERM_Z[((ijk.2 as i32 + dk as i32) & 255) as usize];
        *corner = VECS[(ix ^ iy ^ iz) as usize];
    }
    trilinear_interp_array(&corners, uvw)
}

pub fn turb(p: Vec3, depth: usize) -> f64 {
    (1..depth + 1)
        .map(|i| (1. / i as f64, p * i as f64))
        .fold(0., |accum, (weight, p)| accum + weight * noise(p))
        .abs()
}
