use std::collections::VecDeque;
use std::collections::{hash_map::Entry, HashMap};
use std::intrinsics::likely;
use std::sync::Arc;

use crate::arena::conf::GameConf;
use crate::arena::cosmos_ripper::CosmosRipper;
use crate::arena::defs::{Crd, CrdI, Idx, Tick};
use crate::arena::gnd;
use crate::arena::mob::Mob;
use crate::mind::Mind;
use crate::mob::bio::species::SpeciesPool;
use crate::msgpip::pipe::Output;
use crate::msgpip::MPipe;
use crate::{if_likely, jobs};

pub use super::*;
use super::{MobBox, ReadGuard, Weak, WriteGuard};

pub const CHUNK_WIDTH: usize = 1;
pub const CHUNK_HEIGHT: usize = 16;

pub struct Cosmos {
    pub plate: Matrix<Block, CHUNK_WIDTH, CHUNK_HEIGHT>,
    pub angelos: MajorAngelos,
    ripper: CosmosRipper,
}

pub struct PKey {
    _a: (),
}

#[derive(Default)]
pub struct Block {
    pub ground: Ground,
    mob: Option<Arc<MobBlock>>,
}

pub struct _MobBlock<M: ?Sized> {
    pub at: CrdI,
    on_plate: bool,
    pub mob: M,
}

impl<M> _MobBlock<M> {
    #[inline]
    pub fn new(at: CrdI, mob: M) -> Self {
        Self {
            at,
            on_plate: false,
            mob,
        }
    }
}

impl<M: ?Sized> _MobBlock<M> {
    #[inline]
    pub fn on_plate(&self) -> bool {
        self.on_plate
    }
}

pub type MobBlock = _MobBlock<dyn Mob>;

include!("angelos.rs");
include!("deamon.rs");

pub struct Properties {
    pub tick: u64,
}

impl PKey {
    #[inline]
    fn new() -> Self {
        Self { _a: () }
    }
}

impl Block {
    #[inline]
    pub fn mob(&self) -> Option<(CrdI, Weak<MobBlock>)> {
        self.mob.as_ref().map(|mob| (mob.as_ref().at, mob.into()))
    }
}

impl Cosmos {
    pub fn new(conf: Arc<conf::Conf>) -> Self {
        let plate_size = conf.game.chunk_size * conf.game.chunk_count;
        let plate_size_usize: Coord<usize> = plate_size.try_into().unwrap();
        Cosmos {
            plate: Matrix::new(plate_size_usize),
            angelos: MajorAngelos {
                singletons: Singletons::new(conf.clone()),
                conf: conf.clone(),
                properties: Properties { tick: 0 },
                plate_size,
                pkey: PKey::new(),
                async_data: Mutex::new(MajorAngelosAsyncData {
                    gnd_messages: MPipe::new(),
                    gnd_orders: MPipe::new(),
                    mob_pos_messages: MPipe::new(),
                    mob_pos_orders: MPipe::new(),
                    mob_messages: MPipe::new(),
                    mob_orders: MPipe::new(),
                }),
                mind_waiting_queue: Default::default(),
            },
            ripper: CosmosRipper::new(plate_size, conf.game.chunk_size, conf.game.padding),
        }
    }

    pub fn pk<F: FnOnce(&mut Cosmos, &PKey)>(&mut self, f: F) {
        let pkey = PKey::new();
        f(self, &pkey);
    }

    pub fn set<M: Mob + Unsize<dyn Mob> + ?Sized>(
        &mut self,
        mob: MobBox<M>,
    ) -> Result<Weak<MobBlock>, MobBox<M>> {
        Deamon::set_plate(&mut self.plate, &self.angelos, mob)
    }

    pub fn take<'g, M: Mob + ?Sized>(
        &mut self,
        mob: MobRefMut<'g, M>,
    ) -> Result<MobBox<M>, MobRefMut<'g, M>> {
        Deamon::take_plate(&mut self.plate, &self.angelos, mob)
    }

    pub fn reset<'g, M: Mob + Unsize<dyn Mob> + ?Sized>(
        &mut self,
        mob: &mut MobRefMut<M>,
        new_at: CrdI,
    ) -> Result<(), ()> {
        Deamon::reset_plate(&mut self.plate, &self.angelos, mob, new_at)
    }
}

impl Cosmos {
    #[inline]
    fn pos_to_weak_mob<T: Send + Sync>(
        &self,
        from: Output<Crd, T>,
        to: &mut Output<Weak<MobBlock>, T>,
    ) {
        let thread_count = self.angelos.conf.runtime.thread_count;
        let mut workers = Vec::with_capacity(thread_count);
        for _ in 0..thread_count {
            workers.push(Vec::new());
        }

        let jobs = from.into_iter().collect();
        jobs::work(workers.iter_mut(), jobs, |worker, job| {
            if_likely!(let Some((_pos, weak_mob)) = self.plate[job.0].mob() => {
                worker.push((weak_mob, job.1))
            });
        });

        for worker in workers {
            for (mob, data) in worker {
                to.append(mob, data);
            }
        }
    }

    #[inline]
    fn weak_mob_to_interval<T: Send + Sync>(
        &self,
        mobs: Output<Weak<MobBlock>, T>,
        chunk_size: Crd,
    ) -> HashMap<CrdI, Vec<(Weak<MobBlock>, Vec<T>)>> {
        let thread_count = self.angelos.conf.runtime.thread_count;
        let mut workers: Vec<HashMap<CrdI, Vec<(Weak<MobBlock>, Vec<T>)>>> =
            Vec::with_capacity(thread_count);
        for _ in 0..thread_count {
            workers.push(HashMap::new());
        }

        let jobs = mobs.into_iter().collect();
        ReadGuard::with(&self.angelos.pkey, |guard| {
            jobs::work(workers.iter_mut(), jobs, |worker, job| {
                if_likely!(let Some(mob_ref) = guard.wrap_weak(job.0.clone()) => {
                    let pos = mob_ref.at();
                    let center = Coord((pos.0.from + pos.0.to) / 2, (pos.1.from + pos.1.to) / 2);
                    let center = self.angelos.normalize_pos(center);
                    let left_top = Coord(
                        center.0 - center.0 % chunk_size.0,
                        center.1 - center.1 % chunk_size.1,
                    );
                    let interval = left_top | (left_top + chunk_size - Coord(1, 1));
                    worker.entry(interval).or_default().push(job);
                });
            });
        });

        let mut worker_iter = workers.into_iter();

        let mut result = worker_iter.next().unwrap_or_default();

        for worker in worker_iter {
            for (interval, mut data) in worker.into_iter() {
                result.entry(interval).or_default().append(&mut data);
            }
        }

        result
    }

    #[inline]
    pub(crate) fn message_tick(&mut self) {
        {
            let thread_count = self.angelos.conf.runtime.thread_count;
            let angelos_data = self.angelos.async_data.get_mut().unwrap();
            let gnd_messages = angelos_data.gnd_messages.pop_this_turn();
            let mob_pos_messages = angelos_data.mob_pos_messages.pop_this_turn();
            let mut mob_messages = angelos_data.mob_messages.pop_this_turn();

            let mut workers = Vec::with_capacity(thread_count);
            for _ in 0..thread_count {
                workers.push(self.angelos.make_worker());
            }

            jobs::work(
                workers.iter_mut(),
                gnd_messages.into_iter().collect(),
                |angelos, (pos, msgs)| self.plate[pos].ground.hear(self, angelos, pos, msgs),
            );

            self.pos_to_weak_mob(mob_pos_messages, &mut mob_messages);
            ReadGuard::with(&self.angelos.pkey, |guard| {
                jobs::work(
                    workers.into_iter(),
                    mob_messages.into_iter().collect(),
                    |angelos, (mob, msgs)| {
                        if_likely!(let Some(mob_ref) = guard.wrap_weak(mob) => {
                            mob_ref.hear(self, angelos, msgs, guard);
                        });
                    },
                );
            })
        }
    }

    #[inline]
    pub(crate) fn order_tick(&mut self) {
        let thread_count = self.angelos.conf.runtime.thread_count;
        let angelos_data = self.angelos.async_data.get_mut().unwrap();
        let gnd_orders = angelos_data.gnd_orders.pop_this_turn();
        let mob_pos_orders = angelos_data.mob_pos_orders.pop_this_turn();
        let mut mob_orders = angelos_data.mob_orders.pop_this_turn();

        let mut workers = Vec::with_capacity(thread_count);
        for _ in 0..thread_count {
            workers.push(self.angelos.make_worker());
        }

        jobs::work(
            workers.iter_mut(),
            gnd_orders.into_iter().collect(),
            |angelos, (pos, orders)| {
                #![allow(mutable_transmutes)]
                let mg: &mut gnd::Ground = unsafe { std::mem::transmute(&self.plate[pos].ground) };
                mg.order(pos, angelos, orders);
            },
        );

        self.pos_to_weak_mob(mob_pos_orders, &mut mob_orders);

        let mut chunked_orders = self.weak_mob_to_interval(mob_orders, self.ripper.chunk_size);

        WriteGuard::with(&self.angelos.pkey, |guard| {
            self.ripper.with(|batch| {
                let works: Vec<_> = batch
                    .into_iter()
                    .filter_map(|(chunk, bound)| {
                        if let Entry::Occupied(o) = chunked_orders.entry(chunk) {
                            Some((bound, o.remove_entry().1))
                        } else {
                            None
                        }
                    })
                    .collect();
                jobs::work(
                    workers.iter_mut(),
                    works,
                    |worker, (bound, local_mob_orders)| {
                        #![allow(mutable_transmutes)]
                        let mut deamon = Deamon {
                            angelos: worker,
                            plate: unsafe { std::mem::transmute(&self.plate) },
                            bound,
                        };
                        for (mob, orders) in local_mob_orders.into_iter() {
                            if let Some(mob_ref_mut) = unsafe { guard.wrap_weak_mut(mob) } {
                                mob_ref_mut.order(&mut deamon, orders);
                            }
                        }
                    },
                );
            });
        });
    }

    #[inline]
    pub fn add_tick(&mut self) {
        self.angelos.properties.tick += 1;
    }

    #[inline]
    pub fn flush_minds(&mut self) -> VecDeque<Box<dyn Mind>> {
        self.angelos.flush_minds()
    }
}
