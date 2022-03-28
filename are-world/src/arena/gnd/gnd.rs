use crate::arena::cosmos::Deamon;
use crate::arena::defs::Crd;
use crate::arena::gnd::{Msg, Order};
use crate::arena::*;

pub struct Ground {
    pub name: SWord,
    // pub element: element::Element,
}

impl Ground {
    pub fn hear(&self, cosmos: &Cosmos, self_at: Crd, messages: Vec<Msg>) {
        todo!()
    }

    pub fn order(&mut self, at: Crd, deamon: &Deamon, orders: Vec<Order>) {
        todo!()
    }
}

impl Ground {
    fn refresh_name(&mut self) {}
}

impl Default for Ground {
    fn default() -> Self {
        Self {
            name: SWord::default(),
        }
    }
}
