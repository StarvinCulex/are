use std::sync::Arc;

use rc_box::ArcBox;

use crate::arena::cosmos::{Deamon, MobBlock, PKey};
use crate::arena::mob::{Msg, Order};
use crate::arena::{Cosmos, ReadGuard};
use crate::cosmos::defs::CrdI;
use crate::{Angelos, P, MobRef, MobRefMut};

pub trait Mob: Send + Sync {
    fn into_arc(self) -> Arc<MobBlock>;

    fn into_box(self) -> ArcBox<MobBlock>
    where
        Self: Sized,
    {
        self.into_arc()
            .try_into()
            .unwrap_or_else(|_| unreachable!())
    }

    fn into_block(self) -> P<MobBlock>
    where
        Self: Sized,
    {
        self.into_arc().into()
    }

    fn get_name(&self) -> String;

    fn hear(
        self: MobRef<Self>,
        cosmos: &Cosmos,
        angelos: &mut Angelos,
        message: Vec<Msg>,
        guard: &ReadGuard<PKey>,
    );

    fn order(self: MobRefMut<Self>, deamon: &mut Deamon, order: Vec<Order>);
}
