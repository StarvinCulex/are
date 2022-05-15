use std::sync::Arc;

use rand::distributions::Uniform;
use rand::Rng;

use crate::arena::cosmos::*;
use crate::arena::cosmos::{Deamon, MobBlock, PKey, _MobBlock};
use crate::arena::defs::{Crd, CrdI, Idx};
use crate::arena::mob::{Mob, Msg, Order};
use crate::arena::types::*;
use crate::arena::Weak;
use crate::arena::{Cosmos, MobRef, MobRefMut, ReadGuard};
use crate::lock::spinlock::SpinLock;
use crate::mob::bio::atk::ATK;
use crate::mob::common::bfs::manhattan_carpet_bomb_search;
use crate::mob::common::pathfind::silly_facing;
use crate::{measure_area, measure_length, mob, Coord, Interval};

use super::species::Species;

pub struct Bio {
    pub species: Arc<Species>,
    pub wake_tick: WakeTickT,
    pub energy: EnergyT,
    pub mutex: SpinLock<BioTarget>,
}

pub struct BioTarget {
    pub weight: i8,
    pub target: Option<CrdI>,
    pub target_mob: Option<Weak<MobBlock>>,
}

impl Bio {
    pub fn new(species: Arc<Species>, energy: EnergyT) -> Bio {
        Bio {
            species,
            energy,
            wake_tick: 0,
            mutex: SpinLock::new(BioTarget {
                weight: 0,
                target: None,
                target_mob: None,
            }),
        }
    }

    fn suicide(self: MobRefMut<Self>, deamon: &mut Deamon) {
        let energy: f64 = self.energy.into();
        let size = measure_area(deamon.angelos.major.plate_size.into(), self.at().into());
        let energy_per_grid: EnergyT = (energy
            * deamon
                .angelos
                .major
                .properties
                .runtime_conf
                .corpse_convert_cost
            / (size.0 * size.1) as f64) as EnergyT;
        for (_, g) in deamon.get_ground_iter_mut(self.at()).unwrap() {
            g.plant.add_corpse(energy_per_grid);
        }

        deamon.take(self).unwrap_or_else(|_| unreachable!());
    }
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
        // 维持心跳
        angelos.tell(self.downgrade(), Msg::Wake, self.species.wake_period());
        angelos.order(
            self.downgrade(),
            Order::MobMainTick,
            self.species.act_delay(),
        );

        // 观察周围
        let mut self_target = self.mutex.lock().unwrap();
        if self_target.target.is_none() {
            if let Some(target_mob_weak) = self_target.target_mob.clone() {
                self_target.target = guard.wrap_weak(target_mob_weak).map(|m| m.at());
                if self_target.target.is_none() {
                    self_target.target_mob = None;
                }
            } else {
                let set_move_target = {
                    // 观察周围方格
                    self.wake_tick % self.species.watch_period() == 0 &&
                        manhattan_carpet_bomb_search(
                            self.at().from(),
                            self.species.watch_range() as u16,
                            |p| {
                                let target = self.species.watching_choice(&cosmos.plate[p]);
                                if target.weight == i8::MIN || target.weight == i8::MAX {
                                    *self_target = target;
                                    return Some(());
                                }
                                if target.weight.abs() > self_target.weight.abs() {
                                    *self_target = target;
                                }
                                None
                            },
                        )
                            .is_some()
                }
                    // 闲逛
                    || {
                    self.wake_tick % self.species.stroll_period() == 0 && {
                        let distribute = Uniform::from(-self.species.stroll_range()..self.species.stroll_range() + 1);
                        let relative_target = Coord(
                            angelos.random.sample(distribute),
                            angelos.random.sample(distribute),
                        );
                        let target = angelos.major.normalize_area(self.at().offset(relative_target));
                        *self_target = BioTarget {
                            weight: 0,
                            target: Some(target),
                            target_mob: None,
                        };
                        true
                    }
                };
            }
        }
    }

    fn order(mut self: MobRefMut<Self>, deamon: &mut Deamon, orders: Vec<Order>) {
        let matrix_size = deamon.angelos.major.plate_size;

        let mut main_tick = false;
        for odr in orders {
            match odr {
                Order::MobMainTick => main_tick = true,
            }
        }

        if !main_tick {
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
                    self.suicide(deamon);
                    return;
                }
            }
        }

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
                let child_size = Coord(0, 0) | child_species.size();
                let child = Bio::new(child_species, child_energy);
                let mut pchild = Some(child.into_box());

                // 是否放下了幼崽
                // todo 幼崽的初始大小可能不同
                if let Some(pchild) = {
                    let mut r = None;
                    for child_at in [
                        Coord(-measure_length(matrix_size.0, child_size.0), 0),
                        Coord(0, -measure_length(matrix_size.1, child_size.1)),
                        Coord(measure_length(matrix_size.0, self.at().0), 0),
                        Coord(0, measure_length(matrix_size.1, self.at().1)),
                    ]
                    .into_iter()
                    .map(|x| child_size.offset(self.at().from() + x))
                    {
                        let mut child_mob = pchild.take().unwrap();
                        child_mob.at = deamon.angelos.major.normalize_area(child_at);
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
                }
            }
        }

        // 移动
        if self.wake_tick % self.species.move_period() == 0 {
            if let Some(target) = self.mutex.get_mut().unwrap().target {
                debug_assert_eq!(self.at(), deamon.angelos.major.normalize_area(self.at()));
                let facings = silly_facing(
                    self.at(), // 截至2022/05/15，确定self.at()是归一化的
                    deamon.angelos.major.normalize_area(target),
                    deamon.angelos.major.plate_size,
                );
                let mut move_success = false;
                for facing in facings {
                    let move_target = self.at().offset(
                        facing
                            * if self.mutex.get_mut().unwrap().weight < 0 {
                                Coord(-1, -1)
                            } else {
                                Coord(1, 1)
                            },
                    );
                    if deamon.reset(&mut self, move_target).is_ok() {
                        move_success = true;
                        break;
                    }
                }
                if move_success {
                    self.energy.saturating_sub(self.species.move_cost());
                } else {
                    *self.mutex.get_mut().unwrap() = BioTarget {
                        weight: 0,
                        target: None,
                        target_mob: None,
                    };
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
