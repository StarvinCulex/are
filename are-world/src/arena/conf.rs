use serde::Deserialize;

use crate::arena::defs::{Crd, Tick};
use crate::Coord;

pub struct StaticConf {
    pub plate_size: Crd,
}

#[derive(Deserialize)]
pub struct RuntimeConf {
    pub period: Tick,
    pub fire_tick: Tick,
    pub thread_count: usize,
    pub chunk_size: Crd,
    pub padding: Crd,

    pub plant_aging: f64,
    pub plant_sow: f64,
}
