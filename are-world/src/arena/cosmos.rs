use std::collections::VecDeque;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread;

use rayon::prelude::*;

use crate::arena::conf::StaticConf;
use crate::arena::defs::{Crd, CrdI, Tick};
use crate::arena::gnd;
use crate::arena::mob::Mob;
use crate::msgpip::MPipe;

pub use super::*;
use super::{Weak, P};

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

pub struct MobBlock {
    pub at: CrdI,
    pub mob: dyn Mob,
}

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
    orders: SyncSender<DeamonOrder>,
    results: Mutex<Receiver<DeamonResult>>,
    plate_chan: Mutex<Receiver<Matrix<Block, 1, 1>>>,
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
    fn new() -> Self {
        Self { _a: () }
    }
}

impl Angelos {
    pub fn join(&self, mind: Box<dyn mind::Mind>) {
        self.mind_waiting_queue.lock().unwrap().push_back(mind)
    }
}

enum DeamonOrder {
    Set { mob: Box<MobBlock>, at: CrdI },
    Take { mob: Weak<MobBlock> },
    Reset { mob: Weak<MobBlock>, at: CrdI },
}

enum DeamonResult {
    Set(Result<(), Box<MobBlock>>),
    Take(Result<Box<MobBlock>, ()>),
    Reset(Result<(), ()>),
}

impl<'c> Deamon<'c> {
    fn new(angelos: &'c Angelos, mut plate: Matrix<Block, 1, 1>) -> Self {
        let (orders, orders_receiver) = sync_channel(0);
        let (results_sender, results) = sync_channel(0);
        let (plate_sender, plate_chan) = sync_channel(0);

        thread::spawn(move || {
            'ret: while let Ok(order) = orders_receiver.recv() {
                match order {
                    DeamonOrder::Set { mob, at } => {
                        for (_, grid) in plate.area(at) {
                            if grid.mob.is_some() {
                                results_sender.send(DeamonResult::Set(Err(mob)));
                                continue 'ret;
                            }
                        }
                        results_sender.send(DeamonResult::Set(Ok(())));
                        todo!() // set
                    }
                    DeamonOrder::Take { .. } => {}
                    DeamonOrder::Reset { .. } => {}
                }
            }
            plate_sender.send(plate);
        });

        Self {
            angelos,
            orders,
            results: Mutex::new(results),
            plate_chan: Mutex::new(plate_chan),
        }
    }

    pub fn set(&self, mob: Box<MobBlock>, at: CrdI) -> Result<(), Box<MobBlock>> {
        todo!()
    }

    pub fn take(&self, mob: Weak<MobBlock>) -> Result<Box<MobBlock>, ()> {
        todo!()
    }

    pub fn reset(&self, mob: Weak<MobBlock>, at: CrdI) -> Result<(), ()> {
        todo!()
    }

    fn stop(self) -> Matrix<Block, 1, 1> {
        std::mem::drop(self.orders);
        self.plate_chan.lock().unwrap().recv().unwrap()
    }
}

impl Teller<Crd, gnd::Msg> for Angelos {
    #[inline]
    fn tell(&self, at: Crd, msg: gnd::Msg, delay: Tick) {
        self.gnd_messages.push(delay, at, msg)
    }
}

impl Orderer<Crd, gnd::Order> for Angelos {
    #[inline]
    fn order(&self, at: Crd, order: gnd::Order, delay: Tick) {
        self.gnd_orders.push(delay, at, order)
    }
}

impl Teller<Crd, mob::Msg> for Angelos {
    #[inline]
    fn tell(&self, at: Crd, msg: mob::Msg, delay: Tick) {
        self.mob_pos_messages.push(delay, at, msg)
    }
}

impl Orderer<Crd, mob::Order> for Angelos {
    #[inline]
    fn order(&self, at: Crd, order: mob::Order, delay: Tick) {
        self.mob_pos_orders.push(delay, at, order)
    }
}

impl Teller<Weak<MobBlock>, mob::Msg> for Angelos {
    #[inline]
    fn tell(&self, target: Weak<MobBlock>, msg: mob::Msg, delay: Tick) {
        self.mob_messages.push(delay, target, msg)
    }
}

impl Orderer<Weak<MobBlock>, mob::Order> for Angelos {
    #[inline]
    fn order(&self, target: Weak<MobBlock>, order: mob::Order, delay: Tick) {
        self.mob_orders.push(delay, target, order)
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
    pub(crate) fn message_tick(&mut self) {
        let gnd_messgaes = self.angelos.gnd_messages.pop_this_turn();
        let mob_pos_messages = self.angelos.mob_pos_messages.pop_this_turn();
        let mut mob_messages = self.angelos.mob_messages.pop_this_turn();

        rayon::join(
            || {
                gnd_messgaes
                    .into_iter()
                    .par_bridge()
                    .for_each(|(pos, msgs)| self.plate[pos].ground.hear(self, pos, msgs))
            },
            || {
                mob_pos_messages.into_iter().for_each(|(pos, msgs)| {
                    if let Some(m) = &self.plate[pos].mob {
                        mob_messages.append(m.downgrade(), msgs)
                    }
                })
            },
        );

        mob_messages.into_iter().par_bridge().for_each(|(m, msgs)| {
            if let Some(mob) = m.upgrade() {
                mob.get(self).mob.hear(self, msgs, mob.clone())
            }
        });
    }

    #[inline]
    pub(crate) fn order_tick(&mut self) {
        let gnd_orders = self.angelos.gnd_orders.pop_this_turn();
        let mob_pos_orders = self.angelos.mob_pos_orders.pop_this_turn();
        let mut mob_orders = self.angelos.mob_orders.pop_this_turn();

        let deamon = Deamon::new(&self.angelos, std::mem::take(&mut self.plate));

        rayon::join(
            || {
                gnd_orders.into_iter().par_bridge().for_each(|(pos, orders)| {
                    let g = &self.plate[pos].ground as *const gnd::Ground;
                    let mg = unsafe { &mut *(g as *mut gnd::Ground) };
                    mg.order(pos, &deamon, orders);
                })
            },
            || {
                mob_pos_orders.into_iter().for_each(|(pos, orders)| {
                    if let Some(m) = &self.plate[pos].mob {
                        mob_orders.append(m.downgrade(), orders)
                    }
                })
            },
        );

        mob_orders.into_iter().par_bridge().for_each(|(m, orders)| {
            if let Some(mob) = m.upgrade() {
                unsafe { mob.get_mut(&self.angelos.pkey) }
                    .mob
                    .order(&deamon, orders, mob.clone())
            }
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
