use std::fmt::Debug;
use std::ops::{AddAssign, DivAssign};
use std::sync::atomic::{AtomicU8, Ordering};

use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::Rng;

use crate::arena::cosmos::Deamon;
use crate::arena::defs::Crd;
use crate::arena::types::*;
use crate::arena::{gnd, Angelos, Cosmos, Orderer};
use crate::conf::plant::PlantList;
use crate::{conf, Coord};

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
            self.age = self.age.saturating_sub(angelos.major.conf.plant.corpse.rot);
        } else if self
            .kind
            .map(&angelos.major.conf.plant.fruit.threshold)
            .unwrap_or(&EnergyT::MAX)
            <= &self.age
        {
            self.age = self.age.saturating_sub(
                *self
                    .kind
                    .map(&angelos.major.conf.plant.fruit.cost)
                    .unwrap_or(&0),
            );
            for p in [Coord(-1, 0), Coord(0, -1), Coord(0, 1), Coord(1, 0)] {
                angelos.order(at + p, gnd::Order::PlantSowing(self.kind), 0);
            }
        } else {
            self.age = self.age.saturating_add(
                *self
                    .kind
                    .map(&angelos.major.conf.plant.aging.growth)
                    .unwrap_or(&0),
            )
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
    pub fn random_new(conf: &conf::plant::Conf, rng: &mut StdRng) -> Self {
        let total = conf.sow.items_weight.grass + conf.sow.items_weight.tree;
        let n = rng.sample(Uniform::from(0..total));
        if n < conf.sow.items_weight.grass {
            Kind::Grass
        } else {
            Kind::Tree
        }
    }
    #[inline]
    fn map<'b, U: Debug>(&'_ self, plant_list: &'b PlantList<U>) -> Option<&'b U> {
        match *self {
            Kind::None | Kind::Corpse => None,
            Kind::Grass => Some(&plant_list.grass),
            Kind::Tree => Some(&plant_list.tree),
        }
    }
}
