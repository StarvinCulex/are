use serde::Deserialize;

use crate::arena::defs::{Crd, Tick};
use crate::Coord;

pub struct StaticConf {
    pub plate_size: Coord<usize>,
}

#[derive(Deserialize)]
pub struct RuntimeConf {
    pub period: Tick,
    pub fire_tick: Tick,
    pub thread_count: usize,

    pub plant_aging: f64,
    pub plant_sow: f64,
}
