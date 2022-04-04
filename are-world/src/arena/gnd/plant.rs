use std::ops::{AddAssign, DivAssign};
use std::sync::atomic::Ordering::SeqCst;
use std::sync::atomic::{AtomicU8, Ordering};

use crate::arena::cosmos::Deamon;
use crate::arena::defs::Crd;
use crate::arena::{gnd, Angelos, Cosmos, Orderer};
use crate::Coord;

pub struct Plant {
    pub kind: Kind,
    pub age: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    None,
    Grass,
    Tree,
}

impl Plant {
    #[inline]
    pub fn new(kind: Kind) -> Self {
        Plant { kind, age: 0 }
    }

    #[inline]
    pub fn aging(&mut self, at: Crd, angelos: &Angelos) {
        if self.age >= self.kind.max_age() {
            self.age /= 2;
            for p in [Coord(-1, 0), Coord(0, -1), Coord(0, 1), Coord(1, 0)] {
                angelos.order(at + p, gnd::Order::PlantSowing(self.kind), 0);
            }
        } else {
            self.age += 1;
        }
    }

    #[inline]
    pub fn mow(&mut self, value: u8) {
        if self.age.checked_sub(value).is_none() {
            self.age = 0
        }
    }
}

impl Default for Plant {
    fn default() -> Self {
        Plant::new(Kind::None)
    }
}

impl Kind {
    #[inline]
    fn max_age(&self) -> u8 {
        match self {
            Kind::None => 0,
            Kind::Grass => 16,
            Kind::Tree => 128,
        }
    }
}
