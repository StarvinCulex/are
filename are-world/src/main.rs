#![feature(can_vector)]
#![feature(exclusive_range_pattern)]
#![feature(allocator_api)]
#![feature(const_mut_refs)]
#![feature(unchecked_math)]
#![feature(const_inherent_unchecked_arith)]

use crate::grids::{Coord, Interval};
use crate::sword::SWord;

mod grids;
mod queues;
mod sword;

fn main() {
    assert_eq!(
        Coord(Interval::new(1, 2), Interval::new(3, 4)),
        Coord(1, 3) | Coord(2, 4)
    );
}
