use access::Accessor;
pub use area::Area;
pub use util::{measure_area, measure_length};

use super::{coord::Coord, interval::Interval};

include!("matrix.rs");

pub mod access;
pub mod area;
pub mod mapping;

include!("iter.rs");
include!("scan.rs");

pub mod fmt;
pub mod util;
