use crate::arena::cosmos::{Deamon, MobBlock, PKey};
use crate::arena::mob::{Msg, Order};
use crate::arena::{Cosmos, ReadGuard};
use crate::cosmos::defs::CrdI;
use crate::{SWord, P};
use rc_box::ArcBox;
use std::sync::Arc;

pub trait Mob: Send + Sync {
    fn into_arc(self) -> Arc<MobBlock>;
    
    fn into_box(self) -> ArcBox<MobBlock> where Self: Sized {
        self.into_arc().try_into().unwrap_or_else(|_| unreachable!())
    }
    
    fn into_block(self) -> P<MobBlock> where Self: Sized {
        self.into_arc().into()
    }

    fn get_name(&self) -> String;

    fn hear(&self, cosmos: &Cosmos, message: Vec<Msg>, this: P<MobBlock>, guard: &ReadGuard<PKey>);

    fn order(&mut self, at: CrdI, deamon: &Deamon, order: Vec<Order>, this: P<MobBlock>);
}
