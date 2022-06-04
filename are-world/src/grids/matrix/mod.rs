use access::Accessor;
pub use area::{Area, AreaMut};
pub use util::*;

use super::{coord::Coord, interval::Interval};

include!("matrix.rs");
include!("serde.rs");
pub mod access;
pub mod area;
pub mod mapping;

include!("iter.rs");
include!("scan.rs");
include!("fast.rs");

pub mod fmt;
pub mod util;
