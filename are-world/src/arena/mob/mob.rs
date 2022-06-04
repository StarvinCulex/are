use std::sync::Arc;

use crate::arena::cosmos::{Deamon, MobBlock, PKey};
use crate::arena::mob::{Msg, Order};
use crate::arena::{Cosmos, MobBox, ReadGuard};
use crate::cosmos::defs::CrdI;
use crate::meta::types::ThreatT;
use crate::{Angelos, MobRef, MobRefMut};

pub trait Mob: Send + Sync + ToString {
    fn into_arc(self) -> Arc<MobBlock>;

    fn into_box(self) -> MobBox<dyn Mob>
    where
        Self: Sized,
    {
        let mob = self.into_arc();
        debug_assert_eq!(Arc::strong_count(&mob), 1);
        debug_assert_eq!(mob.on_plate(), false);
        unsafe { MobBox::new_unchecked(mob) }
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

    fn threat(&self) -> ThreatT;
}
