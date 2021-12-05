#![feature(can_vector)]
#![feature(exclusive_range_pattern)]
#![feature(allocator_api)]
#![feature(const_mut_refs)]
#![feature(unchecked_math)]
#![feature(const_inherent_unchecked_arith)]
#![feature(const_fn_trait_bound)]

use crate::grids::*;
use crate::sword::SWord;

mod grids;
mod pipes;
mod sword;

fn main() {
    let matrix = Matrix::<String, 1, 1>::with_ctor(&Coord(10, 10), |opt_pos| {
        if let Some(pos) = opt_pos {
            pos.to_string()
        } else {
            "None".to_string()
        }
    });
    println!("{}", matrix);
    println!("{}", matrix.area(Coord(9, 9) | Coord(0, 0)))
}
