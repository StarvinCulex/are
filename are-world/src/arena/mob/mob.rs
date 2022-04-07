use crate::arena::cosmos::{Deamon, MobBlock, PKey};
use crate::arena::mob::{Msg, Order};
use crate::arena::{Cosmos, ReadGuard};
use crate::cosmos::defs::CrdI;
use crate::{SWord, P};

pub trait Mob: Send + Sync {
    fn into_block(self) -> P<MobBlock>;

    fn get_name(&self) -> String;

    fn hear(&self, cosmos: &Cosmos, message: Vec<Msg>, this: P<MobBlock>, guard: &ReadGuard<PKey>);

    fn order(&mut self, at: CrdI, deamon: &Deamon, order: Vec<Order>, this: P<MobBlock>);
}
