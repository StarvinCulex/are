use std::intrinsics::likely;

use serde::{Deserialize, Serialize};

pub use environment::Environment;
pub use gmsg::Msg;
pub use gorder::Order;
use plant::Kind;
pub use plant::Plant;

use crate::arena::defs::Crd;
use crate::meta::types::EnergyT;
use crate::{Angelos, Cosmos};

pub mod environment;
pub mod gmsg;
pub mod gorder;
pub mod plant;

#[derive(Serialize, Deserialize)]
pub enum Item {
    Air,
    Plant(Plant),
}

impl Default for Item {
    fn default() -> Self {
        Item::Air
    }
}

impl ToString for Item {
    fn to_string(&self) -> String {
        match &self {
            Item::Air => "air".to_string(),
            Item::Plant(p) => p.to_string(),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Ground {
    pub item: Item,
    pub env: Environment,
}

impl Ground {
    pub fn hear(&self, cosmos: &Cosmos, angelos: &mut Angelos, self_at: Crd, messages: Vec<Msg>) {
        match &self.item {
            Item::Air => {}
            Item::Plant(plant) => {}
        }
    }

    pub fn order(&mut self, at: Crd, angelos: &mut Angelos, orders: Vec<Order>) {
        match &mut self.item {
            Item::Air => {}

            Item::Plant(plant) => {
                for order in orders {
                    match order {
                        Order::PlantAging => {
                            plant.aging(at, &mut self.env, angelos);
                        }
                    }
                }
            }
        }
    }

    #[inline]
    pub fn aging(&mut self, at: Crd, angelos: &Angelos) {
        match &mut self.item {
            Item::Air => {}
            Item::Plant(p) => p.aging(at, &mut self.env, angelos),
        }
    }

    #[inline]
    pub fn energy(&self) -> EnergyT {
        match &self.item {
            Item::Air => 0,
            Item::Plant(p) => p.energy,
        }
    }

    #[inline]
    pub fn mow(&mut self, value: EnergyT) -> EnergyT {
        self.mow_threshold(value, 0)
    }

    #[inline]
    pub fn mow_threshold(&mut self, value: EnergyT, threshold: EnergyT) -> EnergyT {
        match &mut self.item {
            Item::Air => 0,
            Item::Plant(p) => p.mow_threshold(value, threshold),
        }
    }
}

impl Ground {
    pub fn name(&self) -> String {
        todo!()
    }
}
