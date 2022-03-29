use std::collections::VecDeque;
use std::sync::mpsc::{channel, sync_channel, Receiver, SyncSender};
use std::thread;

use rayon::prelude::*;
use duplicate::duplicate;

use crate::arena::conf::StaticConf;
use crate::arena::defs::{Crd, CrdI, Tick};
use crate::arena::gnd;
use crate::arena::mob::Mob;
use crate::msgpip::MPipe;
use crate::msgpip::pipe::Output;

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
                                results_sender.send(DeamonResult::Set(Err(mob))).unwrap();
                                continue 'ret;
                            }
                        }
                        results_sender.send(DeamonResult::Set(Ok(()))).unwrap();
                        todo!() // set
                    }
                    DeamonOrder::Take { .. } => {}
                    DeamonOrder::Reset { .. } => {}
                }
            }
            plate_sender.send(plate).unwrap();
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
        k = Matrix::<(), 1, 1>::normalize_pos(self.plate_size.to_isize(), k.to_isize()).to_i16();
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
        rayon::join(|| {
            from.into_iter().par_bridge().for_each_with(tx, |tx, (pos, data)| {
                if let Some(mob) = &self.plate[pos].mob {
                    tx.send((mob.downgrade(), data)).unwrap();
                }
            });
        }, move || {
            for (weak_mob, data) in rx.iter() {
                to.append(weak_mob, data)
            }
        });
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
                    // pos is distinct, so there is no data racing
                    #![allow(mutable_transmutes)]
                    let mg: &mut gnd::Ground = unsafe { std::mem::transmute(&self.plate[pos].ground) };
                    mg.order(pos, &deamon, orders);
                })
            },
            || self.pos_to_weak_mob(mob_pos_orders, &mut mob_orders),
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
