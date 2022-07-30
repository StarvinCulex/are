use plotters::style::RGBColor;

use crate::arena::gnd::plant::Kind;
use crate::arena::gnd::Item;
use crate::{math, Cosmos};

pub fn draw<P: AsRef<std::path::Path> + ?Sized>(
    cosmos: &Cosmos,
    target: &P,
) -> Result<(), Box<dyn std::error::Error>> {
    math::paint(
        &cosmos.plate,
        target,
        &RGBColor(255, 0, 255),
        |b| match &b.ground.item {
            Item::Air => RGBColor(0, 0, 0),
            Item::Plant(p) => match p.kind {
                0 => RGBColor(63, 255, 63),
                1 => RGBColor(31, 127, 31),
                _ => RGBColor(0, 255, 0),
            },
        },
    )
}
