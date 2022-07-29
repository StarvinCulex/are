use crate::arena::gnd::Environment;
use crate::meta::types::*;

pub const DETAIL: [PlantDetail; 2] = [
    PlantDetail {
        max_energy: 50000,
        name: "grass",
        grow: 1000,
    },
    PlantDetail {
        name: "tree",
        max_energy: 500000,
        grow: 2000,
    },
];

pub struct PlantDetail {
    grow: EnergyT,
    pub max_energy: EnergyT,
    pub name: &'static str,
}

impl PlantDetail {
    #[inline]
    pub fn growth(&self, env: &Environment) -> EnergyT {
        todo!()
    }
}
