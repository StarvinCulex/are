use std::sync::Arc;

use rc_box::ArcBox;

use crate::arena::cosmos::*;
use crate::arena::cosmos::{Deamon, MobBlock, PKey, _MobBlock};
use crate::arena::defs::CrdI;
use crate::arena::mob::{Mob, Msg, Order};
use crate::arena::{mob, Cosmos, ReadGuard};
use crate::{Interval, SWord, P};

pub struct Mech {}

impl Mob for Mech {
    fn into_arc(self) -> Arc<MobBlock> {
        todo!()
    }

    fn get_name(&self) -> String {
        "delete this".to_string()
    }

    fn hear(&self, cosmos: &Cosmos, message: Vec<Msg>, this: P<MobBlock>, guard: &ReadGuard<PKey>) {
    }

    fn order(&mut self, at: CrdI, deamon: &Deamon, order: Vec<Order>, this: P<MobBlock>) {
        deamon
            .reset(
                this.downgrade(),
                at.map(|x| Interval::new(x.from + 1, x.from + 1)),
            )
            .unwrap();
        deamon.angelos.order(this.downgrade(), mob::Order::Wake, 1);
    }
}
