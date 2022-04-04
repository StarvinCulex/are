use crate::arena::cosmos::Deamon;
use crate::arena::defs::Crd;
use crate::arena::gnd::plant::{Kind, Plant};
use crate::arena::gnd::{plant, Msg, Order};
use crate::arena::*;

#[derive(Default)]
pub struct Ground {
    // pub element: element::Element,
    pub plant: Plant,
}

impl Ground {
    pub fn hear(&self, cosmos: &Cosmos, self_at: Crd, messages: Vec<Msg>) {
        for msg in messages {
            match msg {}
        }
    }

    pub fn order(&mut self, at: Crd, angelos: &Angelos, orders: Vec<Order>) {
        for order in orders {
            match order {
                Order::PlantMow(value) => self.plant.mow(value),
                Order::PlantAging => self.plant.aging(at, angelos),
                Order::PlantSowing(kind) => {
                    if self.plant.kind == plant::Kind::None {
                        self.plant = Plant::new(kind)
                    }
                }
            }
        }
    }
}

impl Ground {
    pub fn name(&self) -> String {
        match self.plant.kind {
            Kind::None => "".to_string(),
            Kind::Grass => format!("Gr{:02}", self.plant.age),
            Kind::Tree => format!("T{:03}", self.plant.age),
        }
    }
}
