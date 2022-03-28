pub use std::sync::{atomic, Mutex};

//
pub use conf::RuntimeConf;
pub use cosmos::Angelos;
pub use cosmos::Cosmos;
pub use meta::MetaCosmos;
pub use r#ref::Weak;
pub use r#ref::P;

//
pub use crate::grids::{Coord, Interval, Matrix};
pub use crate::sword::SWord;

pub mod conf;
pub mod util;

pub mod cosmos;
mod defs;
pub mod gnd;
pub mod meta;
pub mod mind;
pub mod mob;
pub mod r#ref;
