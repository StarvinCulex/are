use std::sync::Arc;

use rc_box::ArcBox;

use crate::arena::cosmos::*;
use crate::arena::cosmos::{Deamon, MobBlock, PKey, _MobBlock};
use crate::arena::defs::CrdI;
use crate::arena::mob::{Mob, Msg, Order};
use crate::arena::{mob, Cosmos, ReadGuard};
use crate::{Interval, SWord, P, MobRef, MobRefMut};

pub struct Mech {}

impl Mob for Mech {
    fn into_arc(self) -> Arc<MobBlock> {
        todo!()
    }

    fn get_name(&self) -> String {
        "delete this".to_string()
    }

    fn hear(
        self: MobRef<Self>,
        cosmos: &Cosmos,
        angelos: &mut Angelos,
        message: Vec<Msg>,
        guard: &ReadGuard<PKey>,
    ) {
    }

    fn order(self: MobRefMut<Self>, deamon: &mut Deamon, order: Vec<Order>) {
        deamon
            .reset(
                self.downgrade(),
                self.at().map(|x| Interval::new(x.from + 1, x.from + 1)),
            )
            .unwrap();
        deamon.angelos.order(self.downgrade(), mob::Order::Wake, 1);
    }
}
