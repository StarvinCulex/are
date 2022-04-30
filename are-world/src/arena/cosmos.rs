use rc_box::ArcBox;
use std::collections::VecDeque;

use crate::arena::conf::StaticConf;
use crate::arena::defs::{Idx, Crd, CrdI, Tick};
use crate::arena::gnd;
use crate::arena::mob::Mob;
use crate::jobs;
use crate::mind::Mind;
use crate::mob::bio::species::SpeciesPool;
use crate::msgpip::pipe::Output;
use crate::msgpip::MPipe;

pub use super::*;
use super::{ReadGuard, Weak, WriteGuard, P};

pub struct Cosmos {
    pub plate: Matrix<Block, 1, 1>,
    pub angelos: MajorAngelos,
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

include!("angelos.rs");
include!("deamon.rs");

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

impl Cosmos {
    pub fn new(static_conf: StaticConf, runtime_conf: RuntimeConf) -> Self {
        Cosmos {
            plate: Matrix::new(static_conf.plate_size),
            angelos: MajorAngelos {
                properties: Properties {
                    tick: 0,
                    runtime_conf,
                },
                plate_size: static_conf.plate_size,
                pkey: PKey::new(),
                species_pool: SpeciesPool::new(),
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
        }
    }
}

impl Cosmos {
    #[inline]
    fn pos_to_weak_mob<T: Send + Sync>(
        &self,
        from: Output<Crd, T>,
        to: &mut Output<Weak<MobBlock>, T>,
    ) {
        let thread_count = self.angelos.properties.runtime_conf.thread_count;
        let mut workers = Vec::with_capacity(thread_count);
        for _ in 0..thread_count {
            workers.push(Vec::new());
        }

        let jobs = from.into_iter().collect();
        jobs::work(workers.iter_mut(), jobs, |worker, job| {
            if let Some(mob) = &self.plate[job.0].mob {
                worker.push((mob.downgrade(), job.1))
            }
        });

        for worker in workers {
            for (mob, data) in worker {
                to.append(mob, data);
            }
        }
    }

    #[inline]
    pub(crate) fn message_tick(&mut self) {
        {
            let thread_count = self.angelos.properties.runtime_conf.thread_count;
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
                        if let Some(mob) = mob.upgrade() {
                            mob.clone().get(guard).mob.hear(self, angelos, msgs, mob, guard);
                        }
                    },
                );
            })
        }
    }

    #[inline]
    pub(crate) fn order_tick(&mut self) {
        let thread_count = self.angelos.properties.runtime_conf.thread_count;
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
                let mg: &mut gnd::Ground = unsafe { std::mem::transmute(&self.plate[pos].ground) };
                mg.order(pos, angelos, orders);
            },
        );

        self.pos_to_weak_mob(mob_pos_orders, &mut mob_orders);
        todo!()

        // rayon::join(
        //     || {
        //         // &mut plate[pos].ground only
        //         gnd_orders
        //             .into_iter()
        //             .par_bridge()
        //             .for_each(|(pos, orders)| {
        //                 // pos are distinct, so ground is distinct
        //                 #![allow(mutable_transmutes)]
        //                 let mg: &mut gnd::Ground =
        //                     unsafe { std::mem::transmute(&self.plate[pos].ground) };
        //                 mg.order(pos, &self.angelos, orders);
        //             })
        //     },
        //     || {
        //         // &mut plate[pos].mob only
        //         #![allow(mutable_transmutes)]
        //         self.pos_to_weak_mob(mob_pos_orders, &mut mob_orders);
        //         Deamon::with(
        //             &self.angelos,
        //             unsafe { std::mem::transmute(&self.plate) },
        //             |deamon| {
        //                 WriteGuard::with(&self.angelos.pkey, |guard| {
        //                     mob_orders.into_iter().par_bridge().for_each(|(m, orders)| {
        //                         if let Some(mob) = m.upgrade() {
        //                             unsafe { mob.clone().get_mut_unchecked(guard) }.mob.order(
        //                                 mob.at(),
        //                                 &deamon,
        //                                 orders,
        //                                 mob,
        //                             )
        //                         }
        //                     })
        //                 });
        //             },
        //         );
        //     },
        // );
    }

    #[inline]
    pub fn add_tick(&mut self) {
        self.angelos.properties.tick += 1;
    }

    #[inline]
    pub fn flush_minds(&mut self) -> VecDeque<Box<dyn Mind>> {
        self.angelos.flush_minds()
    }

    // // TODO: move this away
    // pub fn burn(&mut self, at: Coord<isize>) {
    //     self.plate[at].ground.element.ignite(self, at);
    // }
}
