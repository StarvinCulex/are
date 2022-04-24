use access::Accessor;
pub use area::{Area, AreaMut};
pub use util::{measure_area, measure_distance, measure_distances, measure_length};

use super::{coord::Coord, interval::Interval};

include!("matrix.rs");

pub mod access;
pub mod area;
pub mod mapping;

include!("iter.rs");
include!("scan.rs");

pub mod fmt;
pub mod util;
