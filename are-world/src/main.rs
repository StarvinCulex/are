#![feature(exclusive_range_pattern)]
#![feature(const_mut_refs)]
#![feature(const_fn_trait_bound)]

use crate::grids::*;
use crate::sword::SWord;

mod grids;
// mod pipes;
mod sword;

fn main() {
    let matrix = Matrix::<String, 2, 2>::with_ctor(&Coord(10, 10), |opt_pos| {
        if let Some(pos) = opt_pos {
            pos.to_string()
        } else {
            "None".to_string()
        }
    });
    println!("{}", matrix);
}
