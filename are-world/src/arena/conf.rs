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
    pub fire_tick: Tick,
    pub thread_count: usize,
}

pub mod plant {
    #[derive(super::Deserialize, Debug)]
    pub struct Conf {
        pub aging: Aging,
        pub sow: Sow,
        pub fruit: Fruit,
        pub corpse: Corpse,
    }

    #[derive(super::Deserialize, Debug)]
    pub struct Aging {
        pub possibility: f64,
        pub growth: PlantList<super::EnergyT>,
    }

    #[derive(super::Deserialize, Debug)]
    pub struct Sow {
        pub possibility: f64,
        pub items_weight: PlantList<usize>,
    }

    #[derive(super::Deserialize, Debug)]
    pub struct Corpse {
        pub rot: super::EnergyT,
        pub convert_rate: f64,
    }

    #[derive(super::Deserialize, Debug)]
    pub struct Fruit {
        pub cost: PlantList<super::EnergyT>,
        pub threshold: PlantList<super::EnergyT>,
    }

    #[derive(super::Deserialize, Debug)]
    pub struct PlantList<V: std::fmt::Debug> {
        pub grass: V,
        pub tree: V,
    }
}
