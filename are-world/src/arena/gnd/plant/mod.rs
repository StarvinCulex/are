use std::fmt::Debug;
use std::intrinsics::{likely, unlikely};
use std::num::NonZeroU8;
use std::ops::{AddAssign, DivAssign};
use std::sync::atomic::{AtomicU8, Ordering};

use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::arena::cosmos::Deamon;
use crate::arena::defs::Crd;
use crate::arena::types::*;
use crate::arena::{gnd, Angelos, Cosmos, Orderer};
use crate::meta::defs::Tick;
use crate::{conf, if_likely, if_unlikely, Coord};

use super::Environment;

pub mod prop;

#[derive(Serialize, Deserialize, Clone)]
pub struct Plant {
    pub kind: Kind,
    pub birthday: Tick,
}

pub type Kind = u8;

impl Plant {
    #[inline]
    pub fn new(kind: Kind, angelos: &Angelos) -> Self {
        let birthday = angelos.major.properties.tick;
        Plant { kind, birthday }
    }

    #[inline]
    pub fn energy(&self, angelos: &Angelos) -> EnergyT {
        let prop = &prop::DETAIL[usize::from(self.kind)];
    }

    #[inline]
    pub fn mow(&mut self, value: EnergyT, tick_now: &Tick) -> EnergyT {
        self.mow_threshold(value, 0, tick_now)
    }

    #[inline]
    pub fn mow_threshold(
        &mut self,
        value: EnergyT,
        threshold: EnergyT,
        tick_now: &Tick,
    ) -> EnergyT {
        let mow = self
            .energy
            .checked_sub(threshold)
            .map(|taking| taking.min(value))
            .unwrap_or_default();
        self.energy -= mow;
        mow
    }
}

impl ToString for Plant {
    fn to_string(&self) -> String {
        prop::DETAIL[usize::from(self.kind)].name.to_string()
    }
}
