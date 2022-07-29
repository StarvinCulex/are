use plotters::prelude::{RGBColor, BLACK};
use rand::prelude::StdRng;

use crate::math::noise::{another_perlin_donut, noise_unify};
use crate::math::paint;
use crate::{Coord, SeedableRng};

#[cfg(test)]
#[test]
fn test_perlin_donut() {
    for i in 2..3 {
        let p = another_perlin_donut(
            &mut StdRng::from_entropy(),
            Coord(1000, 1000),
            Coord(10, 10),
            i,
        );
        let p = noise_unify(p, -1.0..1.0);

        paint(&p, &format!("ap.png"), &BLACK, |&x| {
            if x >= 0.0 {
                RGBColor((x * 256.0 % 256.0) as u8, 0, 0)
            } else {
                RGBColor(0, 0, (-x * 256.0 % 256.0) as u8)
            }
        })
        .unwrap();
    }
}
