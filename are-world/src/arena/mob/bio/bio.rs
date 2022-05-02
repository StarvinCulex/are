use std::sync::Arc;

use crate::arena::cosmos::*;
use crate::arena::cosmos::{Deamon, MobBlock, PKey, _MobBlock};
use crate::arena::defs::{Crd, CrdI};
use crate::arena::mob::{Mob, Msg, Order};
use crate::arena::types::*;
use crate::arena::Weak;
use crate::arena::{Cosmos, ReadGuard, P, MobRef, MobRefMut};
use crate::mob::bio::atk::ATK;
use crate::{matrix, mob, Coord, Interval};

use super::species::Species;

pub struct Bio {
    pub species: Arc<Species>,
    pub wake_tick: WakeTickT,
    pub energy: EnergyT,
    pub target: Target,
    /// 可能的值: (0, 0) (±1, 0) (0, ±1)
    /// 用于order tick里移动，记录移动的偏移量
    pub facing: Crd,
}

#[derive(Clone)]
pub enum Target {
    None,
    Pos {
        weight: TargetWeightT,
        to: Crd,
    },
    Bio {
        weight: TargetWeightT,
        to: Weak<MobBlock>,
    },
}

impl Bio {
    fn suicide(&mut self, at: CrdI, deamon: &Deamon) {}
}

impl Mob for Bio {
    fn into_arc(self) -> Arc<MobBlock> {
        Arc::new(_MobBlock {
            at: CrdI::default(),
            mob: self,
        })
    }

    fn get_name(&self) -> String {
        self.species.name.clone()
    }

    fn hear(
        self: MobRef<Self>,
        cosmos: &Cosmos,
        angelos: &mut Angelos,
        message: Vec<Msg>,
        guard: &ReadGuard<PKey>,
    ) {
        let mut wake = false;
        for msg in message {
            match msg {
                mob::Msg::Wake => wake = true,
            }
        }

        if !wake {
            return;
        }

        angelos.tell(self.downgrade(), Msg::Wake, self.species.wake_span());
    }

    fn order(mut self: MobRefMut<Self>, deamon: &mut Deamon, orders: Vec<Order>) {
        let mut wake = false;
        for odr in orders {
            match odr {
                Order::Wake => wake = true,
            }
        }
        if !wake {
            return;
        }
        self.wake_tick = self.wake_tick.overflowing_add(1).0;

        // 挨饿
        {
            let energy_consume = self.species.wake_energy_consume();
            if self.wake_tick % energy_consume.1 == 0 {
                if let Some(remain) = self.energy.checked_sub(energy_consume.0) {
                    self.energy = remain;
                } else {
                    // 饿死
                    let at = self.at();
                    self.suicide(at, deamon);
                    return;
                }
            }
        }

        deamon
            .angelos
            .order(self.downgrade(), Order::Wake, self.species.wake_span());

        // 能生崽不？
        if self.wake_tick % self.species.breed_period() == 0
            && self.species.spawn_when() <= self.energy
        {
            if let Some(energy_remain) = self.energy.checked_sub(self.species.spawn_energy_cost()) {
                let child_species = deamon
                    .angelos
                    .major
                    .species_pool
                    .clone_species(self.species.clone());
                let child_energy = self.species.spawn_energy();
                let child = Bio {
                    species: child_species,
                    energy: child_energy,
                    wake_tick: 0,
                    target: Target::None,
                    facing: Coord(0, 0),
                };
                let mut pchild = Some(child.into_box());

                // 是否放下了幼崽
                if let Some(pchild) = {
                    let mut r = None;
                    for i in [Coord(0, -1), Coord(0, 1), Coord(1, 0), Coord(-1, 0)] {
                        let child_at = self.at().offset(i * self.species.size());
                        match deamon.set(pchild.take().unwrap(), child_at) {
                            Err(pc) => pchild = Some(pc),
                            Ok(pc) => {
                                r = Some(pc);
                                break;
                            }
                        }
                    }
                    r
                }
                /* then */
                {
                    self.energy = energy_remain;
                    deamon.angelos.tell(
                        pchild.downgrade(),
                        Msg::Wake,
                        self.species.spawn_wake_at(),
                    );
                    deamon.angelos.order(
                        pchild.downgrade(),
                        Order::Wake,
                        self.species.spawn_wake_at(),
                    )
                }
            }
        }

        // 移动
        if self.facing != Coord(0, 0)
            && self.wake_tick % self.species.move_period() == 0
            // 尝试移动
            && deamon
            .reset(self.downgrade(), self.at().offset(self.facing))
            .is_ok()
        {
            self.facing = Coord(0, 0);
            self.energy = self.energy.saturating_sub(self.species.move_cost())
        }
    }
}

impl Bio {
    #[inline]
    fn get_atk(&self) -> ATK {
        todo!()
    }
}
