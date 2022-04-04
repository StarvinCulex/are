use std::sync::Arc;

use crate::arena::cosmos::{Deamon, MobBlock, _MobBlock};
use crate::arena::defs::CrdI;
use crate::arena::mob::{Mob, Msg, Order};
use crate::arena::Cosmos;
use crate::arena::P;
use crate::SWord;

use super::species::Species;

pub struct Bio {
    pub species: Arc<Species>,
}

impl Mob for Bio {
    fn into_block(self) -> P<MobBlock> {
        P::new(_MobBlock::<Bio> {
            at: CrdI::default(),
            mob: self,
        })
    }

    fn get_name(&self) -> SWord {
        todo!()
    }

    fn at(&self) -> CrdI {
        todo!()
    }

    fn hear(&self, cosmos: &Cosmos, message: Vec<Msg>, this: P<MobBlock>) {
        todo!()
    }

    fn order(&mut self, deamon: &Deamon, order: Vec<Order>, this: P<MobBlock>) {
        todo!()
    }
}
