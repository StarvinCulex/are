use std::collections::VecDeque;
use std::sync::mpsc::channel;

use duplicate::duplicate;
use rayon::prelude::*;
use rc_box::ArcBox;

use crate::arena::conf::StaticConf;
use crate::arena::defs::{Crd, CrdI, Tick};
use crate::arena::gnd;
use crate::arena::mob::Mob;
use crate::msgpip::pipe::Output;
use crate::msgpip::MPipe;

pub use super::*;
use super::{ReadGuard, Weak, WriteGuard, P};

pub struct Cosmos {
    pub plate: Matrix<Block, 1, 1>,
    pub angelos: Angelos,
}

pub struct PKey {
    _a: (),
}

#[derive(Default)]
pub struct Block {
    pub ground: gnd::Ground,
    pub mob: Option<P<MobBlock>>,
}

pub struct _MobBlock<M: ?Sized> {
    pub at: CrdI,
    pub mob: M,
}

pub type MobBlock = _MobBlock<dyn Mob>;

pub struct Angelos {
    pub properties: Properties,
    plate_size: Coord<usize>,

    gnd_messages: MPipe<Crd, gnd::Msg>,
    gnd_orders: MPipe<Crd, gnd::Order>,

    mob_pos_messages: MPipe<Crd, mob::Msg>,
    mob_pos_orders: MPipe<Crd, mob::Order>,
    mob_messages: MPipe<Weak<MobBlock>, mob::Msg>,
    mob_orders: MPipe<Weak<MobBlock>, mob::Order>,

    mind_waiting_queue: Mutex<VecDeque<Box<dyn mind::Mind>>>,

    pkey: PKey,
}

pub struct Deamon<'c> {
    pub angelos: &'c Angelos,
    plate: Mutex<Matrix<Block, 1, 1>>,
}

pub trait Teller<Index, Letter> {
    fn tell(&self, index: Index, letter: Letter, delay: Tick);
}

pub trait Orderer<Index, Letter> {
    fn order(&self, index: Index, letter: Letter, delay: Tick);
}

pub struct Properties {
    pub tick: u64,
    pub runtime_conf: RuntimeConf,
}

impl PKey {
    #[inline]
    fn new() -> Self {
        Self { _a: () }
    }
}

impl<WriteKey: ?Sized> P<MobBlock, PKey, WriteKey> {
    #[inline]
    pub fn at(&self) -> CrdI {
        ReadGuard::with(&PKey::new(), |guard| self.get(guard).at)
    }
}

impl Angelos {
    pub fn join(&self, mind: Box<dyn mind::Mind>) {
        self.mind_waiting_queue.lock().unwrap().push_back(mind)
    }
}

impl<'c> Deamon<'c> {
    #[inline]
    fn with<F: FnOnce(&Self)>(
        angelos: &'c Angelos,
        plate: Matrix<Block, 1, 1>,
        f: F,
    ) -> Matrix<Block, 1, 1> {
        let instance = Self {
            angelos,
            plate: Mutex::from(plate),
        };
        f(&instance);
        instance.plate.into_inner().unwrap()
    }

    pub fn set(&self, mob: ArcBox<MobBlock>, at: CrdI) -> Result<(), ArcBox<MobBlock>> {
        let mut plate = self.plate.lock().unwrap();
        // check if the plate is empty
        if plate.area(at).scan().any(|(_, grid)| grid.mob.is_some()) {
            return Err(mob);
        }
        // set the plate
        let mob: P<MobBlock> = mob.into();
        for (_, grid) in plate.area_mut(at) {
            grid.mob = Some(mob.clone());
        }
        Ok(())
    }

    pub fn take(&self, mob: Weak<MobBlock>) -> Result<ArcBox<MobBlock>, ()> {
        let mut plate = self.plate.lock().unwrap();
        let mob = mob.upgrade().ok_or(())?;
        let at = mob.at();
        // check if the mob.at() is valid
        if &mob != plate[at.from()].mob.as_ref().ok_or(())? {
            return Err(());
        }
        let scan = plate.area_mut(at).scan();
        // quick fail: check if it's unique after clearing the plate
        if scan.len() + 1 < mob.strong_count() {
            return Err(())
        }
        // clear the plate
        for (_, grid) in scan {
            grid.mob = None;
        }
        // convert
        mob.try_into_box(&self.angelos.pkey).map_err(|_| unreachable!())
    }

    pub fn reset(&self, mob: Weak<MobBlock>, at: CrdI) -> Result<(), ()> {
        // lock() before upgrade(), as it can be take()-n between upgrade() and lock() otherwise
        let mut plate = self.plate.lock().unwrap();
        let mob = mob.upgrade().ok_or(())?;
        let new_at = mob.at();
        // check if there is another mob
        if plate.area(at).scan().any(|(_, grid)| grid.mob.is_some_with(|pos_mob| pos_mob != &mob)) {
            return Err(());
        }
        // clear the old grids
        for (pos, grid) in plate.area_mut(at) {
            if !new_at.contains(&pos.try_into().unwrap()) {
                grid.mob = None;
            }
        }
        // set the new grids
        for (_, grid) in plate.area_mut(new_at) {
            if grid.mob.is_none() {
                grid.mob = Some(mob.clone());
            }
        }
        Ok(())
    }
}

duplicate! {
    [
        Trait        K                   V               fn_name    holder;
        [ Teller  ]  [ Crd            ]  [ gnd::Msg   ]  [ tell  ]  [ gnd_messages     ];
        [ Orderer ]  [ Crd            ]  [ gnd::Order ]  [ order ]  [ gnd_orders       ];
        [ Teller  ]  [ Crd            ]  [ mob::Msg   ]  [ tell  ]  [ mob_pos_messages ];
        [ Orderer ]  [ Crd            ]  [ mob::Order ]  [ order ]  [ mob_pos_orders   ];
    ]

impl Trait<K, V> for Angelos {
    #[inline]
    fn fn_name(&self, mut k: K, v: V, delay: Tick) {
        k = Matrix::<(), 1, 1>::normalize_pos(self.plate_size.try_into().unwrap(), k.into()).try_into().unwrap();
        self.holder.push(delay, k, v)
    }
}

}

duplicate! {
    [
        Trait        K                   V               fn_name    holder;
        [ Teller  ]  [ Weak<MobBlock> ]  [ mob::Msg   ]  [ tell  ]  [ mob_messages ];
        [ Orderer ]  [ Weak<MobBlock> ]  [ mob::Order ]  [ order ]  [ mob_orders   ];
    ]

impl Trait<K, V> for Angelos {
    #[inline]
    fn fn_name(&self, k: K, v: V, delay: Tick) {
        self.holder.push(delay, k, v)
    }
}

}

impl Cosmos {
    pub fn new(static_conf: StaticConf, runtime_conf: RuntimeConf) -> Self {
        Cosmos {
            plate: Matrix::new(static_conf.plate_size),
            angelos: Angelos {
                properties: Properties {
                    tick: 0,
                    runtime_conf,
                },
                plate_size: static_conf.plate_size,
                mind_waiting_queue: Mutex::default(),
                gnd_messages: MPipe::new(),
                gnd_orders: MPipe::new(),
                mob_pos_messages: MPipe::new(),
                mob_pos_orders: MPipe::new(),
                mob_messages: MPipe::new(),
                mob_orders: MPipe::new(),
                pkey: PKey::new(),
            },
        }
    }
}

impl Cosmos {
    #[inline]
    fn pos_to_weak_mob<T: Send>(&self, from: Output<Crd, T>, to: &mut Output<Weak<MobBlock>, T>) {
        let (tx, rx) = channel();
        rayon::join(
            || {
                from.into_iter()
                    .par_bridge()
                    .for_each_with(tx, |tx, (pos, data)| {
                        if let Some(mob) = &self.plate[pos].mob {
                            tx.send((mob.downgrade(), data)).unwrap();
                        }
                    });
            },
            move || {
                for (weak_mob, data) in rx.iter() {
                    to.append(weak_mob, data)
                }
            },
        );
    }

    #[inline]
    pub(crate) fn message_tick(&mut self) {
        let gnd_messages = self.angelos.gnd_messages.pop_this_turn();
        let mob_pos_messages = self.angelos.mob_pos_messages.pop_this_turn();
        let mut mob_messages = self.angelos.mob_messages.pop_this_turn();

        rayon::join(
            || {
                gnd_messages
                    .into_iter()
                    .par_bridge()
                    .for_each(|(pos, msgs)| self.plate[pos].ground.hear(self, pos, msgs))
            },
            || self.pos_to_weak_mob(mob_pos_messages, &mut mob_messages),
        );

        ReadGuard::with(&self.angelos.pkey, |guard| {
            mob_messages.into_iter().par_bridge().for_each(|(m, msgs)| {
                if let Some(mob) = m.upgrade() {
                    mob.get(guard).mob.hear(self, msgs, mob.clone(), guard)
                }
            })
        });
    }

    #[inline]
    pub(crate) fn order_tick(&mut self) {
        let gnd_orders = self.angelos.gnd_orders.pop_this_turn();
        let mob_pos_orders = self.angelos.mob_pos_orders.pop_this_turn();
        let mut mob_orders = self.angelos.mob_orders.pop_this_turn();

        rayon::join(
            || {
                gnd_orders
                    .into_iter()
                    .par_bridge()
                    .for_each(|(pos, orders)| {
                        #![allow(mutable_transmutes)]
                        let mg: &mut gnd::Ground =
                            unsafe { std::mem::transmute(&self.plate[pos].ground) };
                        mg.order(pos, &self.angelos, orders);
                    })
            },
            || self.pos_to_weak_mob(mob_pos_orders, &mut mob_orders),
        );

        self.plate = Deamon::with(&self.angelos, std::mem::take(&mut self.plate), |deamon| {
            WriteGuard::with(&self.angelos.pkey, |guard| {
                mob_orders.into_iter().par_bridge().for_each(|(m, orders)| {
                    if let Some(mob) = m.upgrade() {
                        unsafe { mob.clone().get_mut_unchecked(guard) }
                            .mob
                            .order(&deamon, orders, mob)
                    }
                })
            });
        });
    }

    // // TODO: move this away
    // pub fn burn(&mut self, at: Coord<isize>) {
    //     self.plate[at].ground.element.ignite(self, at);
    // }
}

impl Angelos {
    pub(crate) fn flush_minds(&mut self) -> VecDeque<Box<dyn mind::Mind>> {
        std::mem::take(self.mind_waiting_queue.get_mut().unwrap())
    }
}
