pub use coord::*;
pub use coord_interval::*;
pub use interval::*;
pub use matrix::*;
pub use matrix::*;

pub mod coord;
pub mod coord_interval;
pub mod interval;
mod isizeconv;
pub mod matrix;

#[macro_export]
macro_rules! o {
    ($x: expr, $y: expr) => {
        Coord($x, $y)
    };
    ($from: expr => $to: expr) => {
        Interval::new($from, $to)
    };
    ($from_x: expr => to_x: expr, $from_y: expr => $to_y: expr) => {
        Coord(Interval::new($from_x, $to_x), Interval::new($from_y, $to_y))
    };
    ($from_x: expr, $from_y: expr => $to_x: expr, $to_y: expr) => {
        Coord(Interval::new($from_x, $to_x), Interval::new($from_y, $to_y))
    };
}
