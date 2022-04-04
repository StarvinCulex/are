use crate::arena::cosmos::{Deamon, MobBlock, PKey};
use crate::arena::defs::CrdI;
use crate::arena::mob::{Msg, Order};
use crate::arena::{Angelos, Cosmos, ReadGuard};
use crate::{SWord, P};

pub trait Mob: Send + Sync {
    fn into_block(self) -> P<MobBlock>;

    fn get_name(&self) -> SWord;

    fn at(&self) -> CrdI;

    fn hear(&self, cosmos: &Cosmos, message: Vec<Msg>, this: P<MobBlock>, reader: &ReadGuard<PKey>);

    fn order(&mut self, deamon: &Deamon, order: Vec<Order>, this: P<MobBlock>);
}
