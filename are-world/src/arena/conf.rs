use serde::Deserialize;

use crate::arena::defs::{Crd, Tick};
use crate::meta::types::*;

#[derive(Deserialize, Debug)]
pub struct Conf {
    pub game: GameConf,
    pub runtime: RuntimeConf,
    pub plant: plant::Conf,
}

#[derive(Deserialize, Debug)]
pub struct GameConf {
    pub chunk_count: Crd,
    pub chunk_size: Crd,
    pub padding: Crd,
}

#[derive(Deserialize, Debug)]
pub struct RuntimeConf {
    pub period: Tick,
    pub thread_count: usize,
}

pub mod plant {
    #[derive(super::Deserialize, Debug)]
    pub struct Conf {
        pub aging_possibility: f64,
        pub sow_possibility: f64,
        pub corpse: Corpse,
        pub grass: Plant,
        pub tree: Plant,
    }

    #[derive(super::Deserialize, Debug)]
    pub struct Plant {
        pub grow: super::EnergyT,
        pub sow_weight: u32,
        pub fruit_cost: super::EnergyT,
        pub fruit_when: super::EnergyT,
    }

    #[derive(super::Deserialize, Debug)]
    pub struct Corpse {
        pub rot: super::EnergyT,
        pub convert_rate: f64,
    }
}
