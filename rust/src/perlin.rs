use rand::prelude::*;

use crate::vec3::Vec3;

fn generate_perm(rng: &mut impl Rng) -> Vec<u8> {
    let size = 256;
    let mut p = Vec::with_capacity(size);
    for i in 0..size {
        p.push(i as u8);
    }

    for i in (1..size).rev() {
        p.swap(i, rng.gen_range(0..i));
    }
    p
}

fn generate_vecs(rng: &mut impl Rng) -> Vec<Vec3> {
    let mut f = Vec::with_capacity(256);
    for _ in 0..256 {
        f.push(Vec3::in_unit_sphere(rng));
    }
    f
}

lazy_static::lazy_static! {
    pub static ref VECS: Vec<Vec3> = generate_vecs(&mut thread_rng());
    pub static ref PERM_X: Vec<u8> = generate_perm(&mut thread_rng());
    pub static ref PERM_Y: Vec<u8> = generate_perm(&mut thread_rng());
    pub static ref PERM_Z: Vec<u8> = generate_perm(&mut thread_rng());
}

#[rustfmt::skip]
fn trilinear_interp(corners: &[[[Vec3; 2]; 2]; 2], uvw: Vec3) -> f64 {
    let uvw3 = uvw * uvw * (Vec3::from(3.) - 2. * uvw);
    let uvw3_inv = Vec3::from(1.) - uvw3;
    corners.iter().flatten().flatten().enumerate()
        .fold(0., |accum, (idx, corner)| {
            let (i, j, k) = (idx & 1, (idx & 2) / 2, (idx & 4) / 4);
            let ijk = Vec3(i as f64, j as f64, k as f64);
            let weight = corner.dot(uvw - ijk);
            let ijk_inv = Vec3::from(1.) - ijk;
            accum + (ijk + uvw3 + ijk_inv * uvw3_inv).reduce(std::ops::Mul::mul) * weight
        })
}

pub fn noise(p: Vec3) -> f64 {
    let ijk = p.map(f64::floor);
    let uvw = p - ijk;
    let mut corners = [[[Vec3::default(); 2]; 2]; 2];
    for di in 0..2 {
        for dj in 0..2 {
            for dk in 0..2 {
                let ix = PERM_X[((ijk.0 as i32 + di as i32) & 255) as usize];
                let iy = PERM_Y[((ijk.1 as i32 + dj as i32) & 255) as usize];
                let iz = PERM_Z[((ijk.2 as i32 + dk as i32) & 255) as usize];
                corners[di][dj][dk] = VECS[(ix ^ iy ^ iz) as usize];
            }
        }
    }
    trilinear_interp(&corners, uvw)
}

pub fn turb(mut p: Vec3, depth: usize) -> f64 {
    let mut accum = 0.;
    let mut weight = 1.;
    for _ in 0..depth {
        accum += weight * noise(p);
        weight *= 0.5;
        p = 2. * p;
    }
    accum.abs()
}
