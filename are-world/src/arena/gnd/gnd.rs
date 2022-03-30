use crate::arena::cosmos::Deamon;
use crate::arena::defs::Crd;
use crate::arena::gnd::plant::Plant;
use crate::arena::gnd::{Msg, Order};
use crate::arena::*;

#[derive(Default)]
pub struct Ground {
    pub name: SWord,
    // pub element: element::Element,
    pub plant: Plant,
}

impl Ground {
    pub fn hear(&self, cosmos: &Cosmos, self_at: Crd, messages: Vec<Msg>) {
        for msg in messages {
            match msg {}
        }
    }

    pub fn order(&mut self, at: Crd, deamon: &Deamon, orders: Vec<Order>) {
        for order in orders {
            match order {
                Order::PlantMow(value) => self.plant.mow(value),
                Order::PlantAging => self.plant.aging(at, deamon),
                Order::PlantSowing(kind) => self.plant = Plant::new(kind),
            }
        }
    }
}

impl Ground {
    fn refresh_name(&mut self) {}
}
