use std::ops::{AddAssign, DivAssign};
use std::sync::atomic::Ordering::SeqCst;
use std::sync::atomic::{AtomicU8, Ordering};

use crate::arena::cosmos::Deamon;
use crate::arena::defs::Crd;
use crate::arena::types::*;
use crate::arena::{gnd, Angelos, Cosmos, Orderer};
use crate::Coord;

pub struct Plant {
    pub kind: Kind,
    pub age: EnergyT,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    None,
    Corpse,
    Grass,
    Tree,
}

impl Plant {
    #[inline]
    pub fn new(kind: Kind) -> Self {
        Plant { kind, age: 0 }
    }

    #[inline]
    pub fn new_corpse(age: EnergyT) -> Self {
        Plant {
            kind: Kind::Corpse,
            age,
        }
    }

    #[inline]
    pub fn aging(&mut self, at: Crd, angelos: &mut Angelos) {
        if self.kind == Kind::Corpse {
            self.age = self
                .age
                .saturating_sub(angelos.major.properties.runtime_conf.corpse_rot);
        } else if self.age >= self.kind.max_age() {
            self.age /= 2;
            for p in [Coord(-1, 0), Coord(0, -1), Coord(0, 1), Coord(1, 0)] {
                angelos.order(at + p, gnd::Order::PlantSowing(self.kind), 0);
            }
        } else {
            self.age += 1;
        }
    }

    #[inline]
    pub fn mow(&mut self, value: EnergyT) -> EnergyT {
        let age_before = self.age;
        self.age = self.age.saturating_sub(value);
        age_before - self.age
    }

    #[inline]
    pub fn add_corpse(&mut self, value: EnergyT) {
        if value > self.age {
            self.kind = Kind::Corpse;
        }
        self.age = self.age.saturating_add(value);
    }
}

impl Default for Plant {
    fn default() -> Self {
        Plant::new(Kind::None)
    }
}

impl Kind {
    #[inline]
    fn max_age(&self) -> EnergyT {
        match self {
            Kind::None => 0,
            Kind::Corpse => 255,
            Kind::Grass => 16,
            Kind::Tree => 128,
        }
    }
}
