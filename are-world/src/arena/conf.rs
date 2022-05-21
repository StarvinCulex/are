use serde::Deserialize;

use crate::arena::defs::{Crd, Tick};
use crate::meta::types::*;

pub struct StaticConf {
    pub chunk_count: Crd,
    pub chunk_size: Crd,
    pub padding: Crd,
}

#[derive(Deserialize)]
pub struct RuntimeConf {
    pub period: Tick,
    pub fire_tick: Tick,
    pub thread_count: usize,

    pub plant_aging: f64,
    pub plant_sow: f64,
    pub corpse_rot: EnergyT,
    pub plant_grow: EnergyT,

    pub corpse_convert_cost: f64,
}
