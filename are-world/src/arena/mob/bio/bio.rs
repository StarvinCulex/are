use std::sync::Arc;

use crate::arena::cosmos::*;
use crate::arena::cosmos::{Deamon, MobBlock, PKey, _MobBlock};
use crate::arena::defs::{Crd, CrdI, Idx};
use crate::arena::mob::{Mob, Msg, Order};
use crate::arena::types::*;
use crate::arena::Weak;
use crate::arena::{Cosmos, MobRef, MobRefMut, ReadGuard};
use crate::mob::bio::atk::ATK;
use crate::mob::common::bfs::manhattan_carpet_bomb_search;
use crate::{measure_area, measure_length, mob, Coord, Interval};

use super::species::Species;

pub struct Bio {
    pub species: Arc<Species>,
    pub wake_tick: WakeTickT,
    pub energy: EnergyT,
    pub mutex: Mutex<BioMutex>,
}

pub struct BioMutex {
    pub target_weight: i8,
    pub target: Target,
}

pub enum Target {
    None,
    Pos(Crd),
    Mob(Weak<MobBlock>),
}

impl Bio {
    pub fn new(species: Arc<Species>, energy: EnergyT) -> Bio {
        Bio {
            species,
            energy,
            wake_tick: 0,
            mutex: Mutex::new(BioMutex {
                target_weight: 0,
                target: Target::None,
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
        let mut lock = self.mutex.lock().unwrap();
        if self.wake_tick % self.species.watch_period() == 0 {
            manhattan_carpet_bomb_search(
                self.at().from(),
                self.species.watch_range() as u16,
                |p| {
                    let (target, weight) = self.species.watching_choice(&cosmos.plate[p]);
                    if weight == i8::MIN || weight == i8::MAX {
                        lock.target_weight = weight;
                        lock.target = target;
                        return Some(());
                    }
                    if weight.abs() > lock.target_weight.abs() {
                        lock.target_weight = weight;
                        lock.target = target;
                    }
                    None
                },
            );

            todo!()
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
                }
            }
        }

        // 移动
        {
            todo!()
            // let at = self.at();
            // let move_step = self.path.pop_front().unwrap_or_default();
            // if move_step != Coord(0, 0) && self.wake_tick % self.species.move_period() == 0
            //     // 尝试移动
            //     && deamon.reset(&mut self, at.offset(move_step)).is_ok()
            // {
            //     self.energy = self.energy.saturating_sub(self.species.move_cost());
            // } else {
            //     self.path.clear();
            //     // 吃草
            //     let eat_starts = self.species.eat_starts();
            //     let eat_takes = self.species.eat_takes();
            //     for (_, g) in deamon.get_ground_iter_mut(at).unwrap() {
            //         if g.plant.age >= eat_starts {
            //             self.energy = self.energy.saturating_add(g.plant.mow(eat_takes));
            //         }
            //     }
            // }
        }
    }
}

impl Bio {
    #[inline]
    fn get_atk(&self) -> ATK {
        todo!()
    }
}
