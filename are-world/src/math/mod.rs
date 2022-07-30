use std::intrinsics::likely;

use plotters::prelude::{BitMapBackend, ChartBuilder, Color, IntoDrawingArea};
use plotters::style::{HSLColor, RGBColor, BLACK, BLUE};
use rand::distributions::{Distribution, Uniform};
use rand::rngs::StdRng;
use rand::Rng;

use crate::{Coord, Interval, Matrix, SeedableRng};

pub mod functions;
pub mod matrix;
pub mod noise;

pub fn paint<
    T,
    C: Color,
    BC: Color,
    F: Fn(&T) -> C,
    P: AsRef<std::path::Path> + ?Sized,
    const CW: usize,
    const CH: usize,
>(
    matrix: &Matrix<T, CW, CH>,
    target: &P,
    background: &BC,
    painter: F,
) -> Result<(), Box<dyn std::error::Error>> {
    const MARGIN: u32 = 0;
    const X_LABEL_AREA_SIZE: u32 = 0;
    const Y_LABEL_AREA_SIZE: u32 = 0;

    let root = BitMapBackend::new(
        target,
        (
            matrix.size().0 as u32 + MARGIN + MARGIN + X_LABEL_AREA_SIZE,
            matrix.size().1 as u32 + MARGIN + MARGIN + Y_LABEL_AREA_SIZE,
        ),
    )
    .into_drawing_area();
    root.fill(background)?;

    let chart = ChartBuilder::on(&root)
        .margin(MARGIN)
        .x_label_area_size(X_LABEL_AREA_SIZE)
        .y_label_area_size(Y_LABEL_AREA_SIZE)
        .build_cartesian_2d(0..matrix.size().0, 0..matrix.size().1)?;
    let plotting_area = chart.plotting_area();
    let range = plotting_area.get_pixel_range();
    let index_map = |p: Coord<isize>| (p.0 as isize, p.1 as isize);

    for (p, v) in matrix.as_area().scan() {
        plotting_area.draw_pixel(index_map(p), &painter(v))?;
    }
    root.present()?;

    Ok(())
}
