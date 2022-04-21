use std::sync::Arc;

use crate::arena::cosmos::{Deamon, MobBlock, PKey, _MobBlock};
use crate::arena::defs::CrdI;
use crate::arena::mob::{Mob, Msg, Order};
use crate::arena::{Cosmos, ReadGuard, P};
use crate::SWord;

use super::species::Species;

pub struct Bio {
    pub species: Arc<Species>,
}

impl Mob for Bio {
    fn into_arc(self) -> Arc<MobBlock> {
        Arc::new(_MobBlock {
            at: CrdI::default(),
            mob: self,
        })
    }

    fn get_name(&self) -> String {
        self.species.name.clone()
    }

    fn hear(&self, cosmos: &Cosmos, message: Vec<Msg>, this: P<MobBlock>, guard: &ReadGuard<PKey>) {
        todo!()
    }

    fn order(&mut self, at: CrdI, deamon: &Deamon, order: Vec<Order>, this: P<MobBlock>) {
        todo!()
    }
}
