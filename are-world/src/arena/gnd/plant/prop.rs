use std::intrinsics::unlikely;

use crate::arena::defs::Tick;
use crate::arena::gnd::Environment;
use crate::meta::types::*;
use crate::{math, MajorAngelos};

pub const DETAIL: [&'static dyn PlantClass; 2] = [
    &FirstPlant {
        name: "grass",
        aging_factor: 10,
        max_energy: 10000,
    },
    &FirstPlant {
        name: "tree",
        aging_factor: 20,
        max_energy: 50000,
    },
];

pub trait PlantClass: ToString {
    fn energy(&self, birthday: &Tick, now: &Tick, env: &Environment) -> EnergyT;
    fn mow_threshold(
        &self,
        value: EnergyT,
        threshold: EnergyT,
        birthday: &mut Tick,
        now: &Tick,
        env: &Environment,
    ) -> EnergyT;

    fn gen_possibility(&self, env: &Environment) -> f32;
}

pub struct FirstPlant {
    pub name: &'static str,
    pub aging_factor: EnergyT,
    pub max_energy: EnergyT,
}

impl ToString for FirstPlant {
    fn to_string(&self) -> String {
        self.name.to_string()
    }
}

impl PlantClass for FirstPlant {
    fn energy(&self, birthday: &Tick, now: &Tick, _: &Environment) -> EnergyT {
        let age = *now - *birthday;

        let (e, overflow) = age.overflowing_mul(self.aging_factor as Tick);
        if unlikely(overflow) || e > self.max_energy as Tick {
            self.max_energy
        } else {
            e as EnergyT
        }
    }

    fn mow_threshold(
        &self,
        value: EnergyT,
        threshold: EnergyT,
        birthday: &mut Tick,
        now: &Tick,
        env: &Environment,
    ) -> EnergyT {
        let energy = self.energy(birthday, now, env);
        let expected_age = if energy >= value && energy - value >= threshold {
            let expected_energy = energy - value;
            expected_energy.div_ceil(self.aging_factor)
        } else if energy <= threshold {
            return 0;
        } else {
            threshold.div_ceil(self.aging_factor)
        };

        *birthday = *now - expected_age as Tick;

        energy - expected_age * self.aging_factor
    }

    fn gen_possibility(&self, env: &Environment) -> f32 {
        0.1
    }
}
