use std::sync;
use std::sync::Arc;

use crate::arena::conf;
use crate::arena::defs::Crd;
use crate::arena::types::*;
use crate::meta::defs::{Idx, Tick};
use crate::mob::bio::bio::{BioAction, BioTarget};
use crate::{Block, Coord};

use super::gene::Gene;

pub struct Species {
    pub name: String,
}

pub struct SpeciesPool {}

impl Species {
    /// 返回值：负数表示逃离，正数表示接近
    pub fn watching_choice(&self, at: Crd, block: &Block) -> BioTarget {
        if block.ground.plant.age >= self.eat_starts() {
            BioTarget {
                action_weight: 1,
                action: BioAction::Eat,
                action_range: Coord(0, 0),
                target: Some(Coord::with_intervals(at, at)),
                target_mob: None,
            }
        } else {
            BioTarget {
                action_weight: 0,
                action: BioAction::Nothing,
                action_range: Default::default(),
                target: None,
                target_mob: None,
            }
        }
    }

    #[inline]
    pub fn stroll_period(&self) -> AgeT {
        1
    }

    #[inline]
    pub fn stroll_range(&self) -> Idx {
        50
    }

    /// 在醒来的第[`u8`]次消耗[`EnergyT`]的能量
    #[inline]
    pub fn wake_energy_consume(&self) -> EnergyT {
        1
    }

    #[inline]
    pub fn breed_period(&self) -> AgeT {
        10
    }

    #[inline]
    pub fn wake_period(&self) -> Tick {
        3
    }

    #[inline]
    pub fn act_delay(&self) -> Tick {
        1
    }

    #[inline]
    pub fn spawn_energy_cost(&self) -> EnergyT {
        150
    }

    #[inline]
    pub fn spawn_energy(&self) -> EnergyT {
        self.spawn_energy_cost() - self.species_energy_value()
    }

    #[inline]
    pub fn species_energy_value(&self) -> EnergyT {
        100
    }

    #[inline]
    pub fn spawn_when(&self) -> EnergyT {
        200
    }

    #[inline]
    pub fn spawn_wake_at(&self) -> Tick {
        1
    }

    /// (0, 0)表示（1, 1)大
    #[inline]
    pub fn size(&self) -> Crd {
        Coord(0, 0)
    }

    #[inline]
    pub fn watch_period(&self) -> AgeT {
        1
    }

    #[inline]
    pub fn watch_range(&self) -> Idx {
        5
    }

    #[inline]
    pub fn move_period(&self) -> AgeT {
        1
    }

    #[inline]
    pub fn move_cost(&self) -> EnergyT {
        1
    }

    #[inline]
    pub fn eat_starts(&self) -> EnergyT {
        10
    }

    pub fn eat_takes(&self) -> EnergyT {
        10
    }
}

impl ToString for Species {
    fn to_string(&self) -> String {
        "species".to_string()
    }
}

impl SpeciesPool {
    pub fn new() -> SpeciesPool {
        SpeciesPool {}
    }

    pub fn clone_species(&self, species: sync::Arc<Species>) -> Arc<Species> {
        species
    }
}
