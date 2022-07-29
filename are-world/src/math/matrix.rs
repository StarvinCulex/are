use plotters::prelude::{BitMapBackend, ChartBuilder, Color, IntoDrawingArea};

use crate::{Coord, Matrix};

pub fn convolute<E, K, O, const CW: usize, const CH: usize, const KCW: usize, const KCH: usize>(
    matrix: &Matrix<E, CW, CH>,
    kernel: &Matrix<K, KCW, KCH>,
) -> Result<Matrix<O, 1, 1>, ()>
where
    O: std::iter::Sum + Clone,
    E: std::ops::Mul<K, Output = O> + Clone,
    K: Clone,
{
    let new_size = *matrix.size() - *kernel.size();
    let new_size: Coord<usize> = new_size.try_into().or(Err(()))?;
    Ok(Matrix::with_ctor(new_size, |pos| {
        kernel
            .as_area()
            .iter()
            .map(|(p, v)| {
                let u = matrix[p + pos].clone();
                u * v.clone()
            })
            .sum()
    }))
}

pub fn convolute_donut<
    E,
    K,
    O,
    const CW: usize,
    const CH: usize,
    const KCW: usize,
    const KCH: usize,
>(
    matrix: &Matrix<E, CW, CH>,
    kernel: &Matrix<K, KCW, KCH>,
) -> Matrix<O, 1, 1>
where
    O: std::iter::Sum + Clone,
    E: std::ops::Mul<K, Output = O> + Clone,
    K: Clone,
{
    let new_size: Coord<usize> = (*matrix.size()).try_into().or(Err(())).unwrap();
    Matrix::with_ctor(new_size, |pos| {
        kernel
            .as_area()
            .iter()
            .map(|(p, v)| {
                let u = matrix[p + pos - *kernel.size() / Coord(2, 2)].clone();
                u * v.clone()
            })
            .sum()
    })
}
