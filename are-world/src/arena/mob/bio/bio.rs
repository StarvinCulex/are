use std::collections::VecDeque;
use std::sync::Arc;

use crate::arena::cosmos::*;
use crate::arena::cosmos::{Deamon, MobBlock, PKey, _MobBlock};
use crate::arena::defs::{Crd, CrdI};
use crate::arena::mob::{Mob, Msg, Order};
use crate::arena::types::*;
use crate::arena::Weak;
use crate::arena::{Cosmos, MobRef, MobRefMut, ReadGuard, P};
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
    pub path: VecDeque<Crd>,
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
                    path: VecDeque::new(),
                };
                let mut pchild = Some(child.into_box());

                // 是否放下了幼崽
                if let Some(pchild) = {
                    let mut r = None;
                    for i in [Coord(0, -1), Coord(0, 1), Coord(1, 0), Coord(-1, 0)] {
                        let mut child_mob = pchild.take().unwrap();
                        let child_at = self.at().offset(i * self.species.size());
                        child_mob.at = child_at;
                        match deamon.set(child_mob) {
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
                    deamon
                        .angelos
                        .tell(pchild.clone(), Msg::Wake, self.species.spawn_wake_at());
                    deamon
                        .angelos
                        .order(pchild, Order::Wake, self.species.spawn_wake_at())
                }
            }
        }

        // 移动
        {
            let at = self.at();
            let move_step = self.path.front().copied().unwrap_or_default();
            if move_step != Coord(0, 0) && self.wake_tick % self.species.move_period() == 0
                // 尝试移动
                && deamon.reset(&mut self, at.offset(move_step)).is_ok()
            {
                self.energy = self.energy.saturating_sub(self.species.move_cost());
                self.path.pop_front();
            } else {
                // 吃草
                let eat_starts = self.species.eat_starts();
                let eat_takes = self.species.eat_takes();
                for (_, g) in deamon.get_ground_iter_mut(at).unwrap() {
                    if g.plant.age >= eat_starts {
                        self.energy = self.energy.saturating_add(g.plant.mow(eat_takes));
                    }
                }
            }
        }
    }
}

impl Bio {
    #[inline]
    fn get_atk(&self) -> ATK {
        todo!()
    }
}
