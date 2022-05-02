use std::sync;
use std::sync::Arc;

use crate::arena::conf;
use crate::arena::defs::Crd;
use crate::arena::types::*;
use crate::meta::defs::{Idx, Tick};

use super::gene::Gene;

pub struct Species {
    pub name: String,
    pub gene: Gene,
}

pub struct SpeciesPool {}

impl Species {
    /// 在醒来的第[`u8`]次消耗[`EnergyT`]的能量
    pub fn wake_energy_consume(&self) -> (EnergyT, WakeTickT) {
        todo!()
    }

    pub fn breed_period(&self) -> WakeTickT {
        todo!()
    }

    pub fn wake_span(&self) -> Tick {
        todo!()
    }

    pub fn spawn_energy_cost(&self) -> EnergyT {
        todo!()
    }

    #[inline]
    pub fn spawn_energy(&self) -> EnergyT {
        self.spawn_energy_cost() - self.energy_cost()
    }

    pub fn energy_cost(&self) -> EnergyT {
        todo!()
    }

    pub fn spawn_when(&self) -> EnergyT {
        todo!()
    }

    pub fn spawn_wake_at(&self) -> u64 {
        todo!()
    }

    pub fn speed(&self) -> SpeedT {
        todo!()
    }

    pub fn size(&self) -> Crd {
        todo!()
    }

    pub fn watch_period(&self) -> WakeTickT {
        todo!()
    }

    pub fn watch_cost(&self) -> EnergyT {
        todo!()
    }

    pub fn watch_range(&self) -> Idx {
        todo!()
    }

    pub fn track_range(&self) -> Idx {
        todo!()
    }

    pub fn move_period(&self) -> WakeTickT {
        todo!()
    }

    pub fn move_cost(&self) -> EnergyT {
        todo!()
    }

    pub fn eat_starts(&self) -> EnergyT {
        todo!()
    }

    pub fn eat_takes(&self) -> EnergyT {
        todo!()
    }
}

impl SpeciesPool {
    pub fn new() -> SpeciesPool {
        SpeciesPool {}
    }

    pub fn clone_species(&self, species: sync::Arc<Species>) -> Arc<Species> {
        todo!()
    }
}
