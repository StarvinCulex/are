use std::ops::Index;

use super::{coord::Coord, interval::Interval};

mod test;

include!("matrix.rs");

include!("access.rs");
include!("iter.rs");

include!("scan.rs");

include!("util.rs");
include!("fmt.rs");
