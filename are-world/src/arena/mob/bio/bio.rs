use std::fmt::{Display, Formatter};
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
use crate::{displacement, measure_area, measure_length, mob, Coord, Interval};

use super::species::Species;

pub struct Bio {
    pub species: Arc<Species>,
    pub age: AgeT,
    pub energy: EnergyT,
    pub target: SpinLock<BioMutex>,
}

pub struct BioMutex {
    pub target: BioTarget,
}

pub struct BioTarget {
    pub action_weight: i8,
    pub action: BioAction,
    // 切比雪夫距离
    pub action_range: Crd,
    pub target: Option<CrdI>,
    pub target_mob: Option<Weak<MobBlock>>,
}

#[derive(Debug)]
pub enum BioAction {
    Nothing,
    Stroll,
    Eat,
}

impl BioTarget {
    pub const fn new() -> BioTarget {
        BioTarget {
            action_weight: 0,
            action: BioAction::Nothing,
            action_range: Coord(0, 0),
            target: None,
            target_mob: None,
        }
    }
}

impl Bio {
    pub fn new(species: Arc<Species>, energy: EnergyT) -> Bio {
        Bio {
            species,
            energy,
            age: 0,
            target: SpinLock::new(BioMutex {
                target: BioTarget::new(),
            }),
        }
    }

    fn suicide(self: MobRefMut<Self>, deamon: &mut Deamon) {
        let energy: f64 = self.energy.into();
        let size = measure_area(deamon.angelos.major.plate_size, self.at());
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
        let mut self_mutex = self.target.lock().unwrap();
        let self_target = &mut self_mutex.target;
        if self_target.target.is_none() {
            if let Some(target_mob_weak) = self_target.target_mob.clone() {
                self_target.target = guard.wrap_weak(target_mob_weak).map(|m| m.at());
                if self_target.target.is_none() {
                    self_target.target_mob = None;
                }
            } else {
                let _: bool = {
                    // 观察周围方格
                    self.age % self.species.watch_period() == 0
                        && manhattan_carpet_bomb_search(
                            self.at().from(),
                            self.species.watch_range() as u16,
                            |p| {
                                let target = self.species.watching_choice(p, &cosmos.plate[p]);
                                if target.action_weight == i8::MIN
                                    || target.action_weight == i8::MAX
                                {
                                    *self_target = target;
                                    return Some(());
                                }
                                if target.action_weight.abs() > self_target.action_weight.abs() {
                                    *self_target = target;
                                }
                                None
                            },
                        )
                        .is_some()
                } || {
                    // 这是运算符[`or`]
                    // 闲逛
                    self.age % self.species.stroll_period() == 0 && {
                        let distribute = Uniform::from(
                            -self.species.stroll_range()..self.species.stroll_range() + 1,
                        );
                        let relative_target = Coord(
                            angelos.random.sample(distribute),
                            angelos.random.sample(distribute),
                        );
                        let target = angelos
                            .major
                            .normalize_area(self.at().offset(relative_target));
                        *self_target = BioTarget {
                            action_weight: 0,
                            action: BioAction::Stroll,
                            action_range: Coord(0, 0),
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
        self.age = self.age.overflowing_add(1).0;

        // 挨饿
        {
            if let Some(remain) = self.energy.checked_sub(self.species.wake_energy_consume()) {
                self.energy = remain;
            } else {
                // 饿死
                let at = self.at();
                self.suicide(deamon);
                return;
            }
        }

        // 能生崽不？
        if self.age % self.species.breed_period() == 0 && self.species.spawn_when() <= self.energy {
            if let Some(energy_remain) = self.energy.checked_sub(self.species.spawn_energy_cost()) {
                let child_species = deamon
                    .angelos
                    .major
                    .singletons
                    .species_pool
                    .clone_species(self.species.clone());
                let child_energy = self.species.spawn_energy();
                let child_size = Coord(0, 0) | child_species.size();
                let child = Bio::new(child_species, child_energy);
                let mut pchild = Some(child.into_box());

                // 是否放下了幼崽
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

        let at = self.at();
        let species = self.species.clone();
        let age = self.age;
        let self_mutex = self.target.get_mut().unwrap();
        let target = &mut self_mutex.target;
        if let Some(target_pos) = target.target {
            debug_assert_eq!(at, deamon.angelos.major.normalize_area(at));
            // 是否可以做动作
            if displacement(deamon.angelos.major.plate_size, at, target_pos).map(Idx::abs)
                <= target.action_range
            {
                // 做动作
                match target.action {
                    // 吃
                    BioAction::Eat => {
                        let mut takes: EnergyT = 0;
                        if let Ok(grounds) = deamon.get_ground_iter_mut(target_pos) {
                            for (p, g) in grounds {
                                takes = takes.saturating_add(g.plant.mow(species.eat_takes()));
                            }
                        }
                        if takes == 0 {
                            *target = BioTarget::new();
                        } else {
                            self.energy = self.energy.saturating_add(takes);
                        }
                    }
                    BioAction::Nothing | BioAction::Stroll => {
                        *target = BioTarget::new();
                    }
                }
            } else if age % species.move_period() == 0 {
                {
                    // 移动
                    let facings = silly_facing(
                        at, // 截至2022/05/15，确定self.at()是归一化的
                        deamon.angelos.major.normalize_area(target_pos),
                        deamon.angelos.major.plate_size,
                    );
                    let mut move_success = false;
                    let action_weight = target.action_weight;
                    for facing in facings {
                        let move_target = at.offset(
                            facing
                                * if action_weight < 0 {
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
                        self.energy = self.energy.saturating_sub(self.species.move_cost());
                    } else {
                        // 移动失败就取消目标
                        self.target.get_mut().unwrap().target = BioTarget::new();
                    }
                }
            }
        }
    }
}

impl ToString for Bio {
    fn to_string(&self) -> String {
        format!(
            "{species}#{ptr:p} A{age} E{energy} T{target}",
            species = self.species.to_string(),
            ptr = self,
            age = self.age,
            energy = self.energy,
            target = self.target.lock().unwrap().target,
        )
    }
}

impl Display for BioTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{W{weight} A{action:?} R{range} T{target}}}",
            weight = self.action_weight,
            action = self.action,
            range = self.action_range,
            target = self
                .target
                .map(|x| x.to_string())
                .unwrap_or_else(|| "-".to_string()),
        )
    }
}
