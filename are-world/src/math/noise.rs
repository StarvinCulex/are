use std::intrinsics::unlikely;
use std::ops::Range;

use rand::distributions::Uniform;
use rand::Rng;

use crate::{Coord, Matrix};

/// 时间复杂度：O( `size.0` * `size.1` * (1 + `mesh_size`<sup>2</sup>) )  
/// 建议使用 `mesh_size`=`2`
pub fn another_perlin_donut<RNG: Rng>(
    rng: &mut RNG,
    size: Coord<usize>,
    chunk_count: Coord<usize>,
    mesh_size: u8,
) -> Matrix<f32, 1, 1> {
    let chunk_size: Coord<f32> = Coord(
        size.0 as f32 / chunk_count.0 as f32,
        size.1 as f32 / chunk_count.1 as f32,
    );

    let mesh_height: f32 = 1.0;

    let lattices = {
        let h_dist = Uniform::new(mesh_height, mesh_height * (mesh_size as f32 + 2.0));
        let r_dist = Uniform::new(0.0f32, 1.0f32);
        let a_dist = Uniform::new(0.0f32, std::f32::consts::PI * 2.0);
        Matrix::<Coord<f32>, 1, 1>::with_ctor(chunk_count, |_| {
            let r = rng.sample(r_dist);
            let a = rng.sample(a_dist);
            let (sin_a, cos_a) = a.sin_cos();
            Coord(sin_a * r, cos_a * r)
        })
    };

    let dots = {
        let mesh_size: isize = mesh_size.into();

        Matrix::<f32, 1, 1>::with_ctor(size, |pos| {
            let chunk_in = (pos.map(|x| x as f32) / chunk_size).map(|x| x as isize);
            let mut sum = 0f32;

            for ly in -mesh_size..mesh_size + 2 {
                for lx in -mesh_size..mesh_size + 2 {
                    let lattice_index = chunk_in + Coord(lx, ly);
                    let lattice_val = lattices[lattice_index];
                    let lattice_at = lattice_index.map(|x| x as f32) * chunk_size;

                    let delta = lattice_at - pos.map(|x| x as f32);
                    let delta_len = delta.dot(delta).sqrt();
                    let delta_unit = delta / Coord(delta_len, delta_len);

                    fn mountain(a: f32) -> f32 {
                        a * (1.0 - a).exp()
                    }
                    let v = {
                        let v: f32 = delta_unit.dot(lattice_val)
                            * mountain(delta_len / chunk_size.dot(chunk_size).sqrt() * 4.0);
                        if !v.is_nan() {
                            v
                        } else {
                            lattice_val.dot(lattice_val).sqrt() * std::f32::consts::E
                                / 4.0
                                / chunk_size.dot(chunk_size).sqrt()
                        }
                    };

                    sum += v;
                }
            }
            sum
        })
    };

    dots
}

// -1..1
#[inline]
pub fn noise_unify(mut matrix: Matrix<f32, 1, 1>, range: Range<f32>) -> Matrix<f32, 1, 1> {
    for (_, value) in matrix.as_area_mut() {
        let x = *value;
        let n = ((x + 1.0) / 2.0).floor();
        let normalized = (x - 2.0 * n) * (n.rem_euclid(2.0) - 0.5) * 2.0;
        *value = (normalized + 1.0) * (range.end - range.start) / 2.0 + range.start;
    }
    matrix
}
