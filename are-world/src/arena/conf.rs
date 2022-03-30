use crate::arena::defs::{Crd, Tick};
use crate::Coord;

pub struct StaticConf {
    pub plate_size: Coord<usize>,
}

pub struct RuntimeConf {
    pub period: Tick,
    pub fire_tick: Tick,

    pub plant_aging: f64,
    pub plant_sow: f64,
}
