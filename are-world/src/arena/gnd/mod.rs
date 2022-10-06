use std::intrinsics::likely;

use serde::{Deserialize, Serialize};

pub use environment::Environment;
pub use gmsg::Msg;
pub use gorder::Order;
use plant::Kind;
pub use plant::Plant;

use crate::arena::defs::{Crd, Tick};
use crate::meta::types::EnergyT;
use crate::{Angelos, Cosmos, MajorAngelos};

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

            Item::Plant(plant) => {}
        }
    }

    #[inline]
    pub fn energy(&self, angelos: &MajorAngelos) -> EnergyT {
        match &self.item {
            Item::Air => 0,
            Item::Plant(p) => plant::prop::DETAIL[p.kind as usize].energy(
                &p.birthday,
                &angelos.properties.tick,
                &self.env,
            ),
        }
    }

    #[inline]
    pub fn mow(&mut self, angelos: &MajorAngelos, value: EnergyT) -> EnergyT {
        self.mow_threshold(angelos, value, 0)
    }

    #[inline]
    pub unsafe fn raw_mow(&mut self, now: &Tick, value: EnergyT, threshold: EnergyT) -> EnergyT {
        match &mut self.item {
            Item::Air => 0,
            Item::Plant(p) => plant::prop::DETAIL[p.kind as usize].mow_threshold(
                value,
                threshold,
                &mut p.birthday,
                now,
                &self.env,
            ),
        }
    }

    #[inline]
    pub fn mow_threshold(
        &mut self,
        angelos: &MajorAngelos,
        value: EnergyT,
        threshold: EnergyT,
    ) -> EnergyT {
        unsafe { self.raw_mow(&angelos.properties.tick, value, threshold) }
    }
}

impl ToString for Ground {
    fn to_string(&self) -> String {
        match &self.item {
            Item::Air => String::from("air"),
            Item::Plant(p) => p.to_string(),
        }
    }
}
