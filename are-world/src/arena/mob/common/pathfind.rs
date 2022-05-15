use std::cmp::Ordering;

use crate::arena::defs::{Crd, CrdI};
use crate::{displacement, Coord};

pub const SILLY_FACING_RETRY: usize = 2;

#[inline]
pub fn silly_facing(mob_at: CrdI, target: CrdI, plate_size: Crd) -> [Crd; SILLY_FACING_RETRY] {
    let delta = displacement(plate_size, mob_at, target);
    let sign = (delta).map(|index| match index.cmp(&0) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    });
    let facing_abs = if delta.0.abs() > delta.1.abs() {
        Coord(2, 1)
    } else {
        Coord(1, 2)
    };
    let facing = sign * facing_abs;
    match facing {
        Coord(2, 0) | Coord(2, 1) => [Coord(1, 0), Coord(0, 1)],
        Coord(1, 2) => [Coord(0, 1), Coord(1, 0)],
        Coord(0, 2) | Coord(-1, 2) => [Coord(0, 1), Coord(-1, 0)],
        Coord(-2, 1) => [Coord(-1, 0), Coord(0, 1)],
        Coord(-2, 0) | Coord(-2, -1) => [Coord(-1, 0), Coord(0, -1)],
        Coord(-1, -2) => [Coord(0, -1), Coord(-1, 0)],
        Coord(0, -2) | Coord(1, -2) => [Coord(0, -1), Coord(1, 0)],
        Coord(2, -1) => [Coord(1, 0), Coord(0, -1)],
        _ => [Coord(0, 0), Coord(0, 0)],
    }
}
