use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::intrinsics::{likely, unlikely};
use std::ops::Add;
use std::sync;
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, Weak};

use rand::distributions::Uniform;
use rand::Rng;

use crate::arena::conf;
use crate::arena::defs::Crd;
use crate::arena::types::*;
use crate::conf::bio::Acid;
use crate::lock::spinlock::{Guard, SpinLock};
use crate::meta::defs::{Idx, Tick};
use crate::mob::bio::atk::ATK;
use crate::mob::bio::bio::{BioAction, BioTarget};
use crate::{
    if_likely, if_unlikely, AngelosStat, Block, Coord, Deamon, MajorAngelos, PKey, ReadGuard, Stats,
};

use super::gene::Gene;

#[derive(Clone)]
pub struct Species {
    pub gene: Gene,
    pub name: String,

    pub wake_period: Tick,
    pub wake_energy_consume: EnergyT,
    pub energy_cost: EnergyT,
    pub spawn_loss: EnergyT,
    pub spawn_init_energy: EnergyT,
    pub incubation_delay: Tick,
    pub size: Crd,
    pub watch_period: AgeT,
    pub watch_range: u16,
    pub move_period: AgeT,
    pub move_cost: EnergyT,
    pub eat_threshold: EnergyT,
    pub eat_takes: EnergyT,

    pub max_hp: HitPointT,
    pub threat: ThreatT,
    pub atk: ATK,
    pub attack_cost: EnergyT,
    pub flee_threshold: ThreatT,
    pub fight_back_threshold: ThreatT,
    pub attack_range: Crd,
    pub chase_threshold: ThreatT,
    pub regeneration: HitPointT,
    pub regeneration_cost: EnergyT,
}

pub struct SpeciesPool {
    snapshot: SpinLock<Option<Arc<Vec<(Gene, Weak<Species>)>>>>,
    conf: Arc<conf::Conf>,
    species_list: SpinLock<HashMap<Gene, Weak<Species>>>,
}

impl Species {
    pub fn bio_count(species: Weak<Species>) -> usize {
        species.strong_count()
    }

    /// 返回值：负数表示逃离，正数表示接近
    #[inline]
    pub fn watching_choice(
        &self,
        angelos: &MajorAngelos,
        at: Crd,
        block: &Block,
        guard: &ReadGuard<PKey>,
    ) -> BioTarget {
        let mob_check = || -> Option<BioTarget> {
            let mob = block.mob_ref(guard)?;
            let threat = mob.threat();
            if threat >= self.flee_threshold {
                Some(BioTarget {
                    action_weight: i8::MIN,
                    action: BioAction::Flee,
                    action_range: if threat <= self.fight_back_threshold {
                        self.attack_range
                    } else {
                        Coord(0, 0)
                    },
                    target: Some(mob.at()),
                    target_mob: Some(mob.downgrade()),
                })
            } else if threat <= self.chase_threshold {
                Some(BioTarget {
                    action_weight: (self.chase_threshold - threat)
                        .try_into()
                        .unwrap_or(i8::MAX),
                    action: BioAction::Chase,
                    action_range: self.attack_range,
                    target: Some(mob.at()),
                    target_mob: Some(mob.downgrade()),
                })
            } else {
                None
            }
        };
        if let Some(x) = mob_check() {
            return x;
        }

        if block.ground.energy(angelos) >= self.eat_threshold {
            BioTarget {
                action_weight: 1,
                action: BioAction::Eat,
                action_range: Coord(0, 0),
                target: Some(Coord::with_intervals(at, at)),
                target_mob: None,
            }
        } else {
            BioTarget {
                action_weight: 0,
                action: BioAction::Nothing,
                action_range: Default::default(),
                target: None,
                target_mob: None,
            }
        }
    }

    #[inline]
    pub fn stroll_period(&self) -> AgeT {
        1
    }

    #[inline]
    pub fn stroll_range(&self) -> Idx {
        50
    }

    #[inline]
    pub fn breed_period(&self) -> AgeT {
        10
    }

    #[inline]
    pub fn act_delay(&self) -> Tick {
        0
    }

    #[inline]
    pub fn spawn_energy_cost(&self) -> EnergyT {
        self.spawn_loss + self.energy_cost + self.spawn_init_energy
    }

    #[inline]
    pub fn spawn_when(&self) -> EnergyT {
        200
    }
}

impl ToString for Species {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl SpeciesPool {
    pub fn new(conf: Arc<conf::Conf>) -> SpeciesPool {
        SpeciesPool {
            snapshot: Default::default(),
            conf,
            species_list: SpinLock::default(),
        }
    }

    pub fn snapshot(&self) -> Arc<Vec<(Gene, Weak<Species>)>> {
        let mut snapshot_guard = self.snapshot.lock().unwrap();
        if let Some(x) = &*snapshot_guard {
            return x.clone();
        }
        let species_guard = self.species_list.lock().unwrap();
        let mut x = Vec::with_capacity(species_guard.len());
        for (g, s) in species_guard.iter() {
            x.push((g.clone(), s.clone()));
        }
        let x = Arc::new(x);
        *snapshot_guard = Some(x.clone());
        x
    }

    pub fn insert_mutate<R: Rng>(mut gene: Gene, rng: &mut R, conf: &conf::Conf) -> Gene {
        let insert_at = rng.gen_range(0..gene.len() + 1);
        let insert_value = {
            let total: usize = conf
                .bio
                .acids
                .iter()
                .map(|(_, acid)| acid.mutate_rate)
                .sum();
            let mut roll = rng.gen_range(0..total);
            let mut chosen_acid = None;
            for (acid_name, acid) in conf.bio.acids.iter() {
                if roll < acid.mutate_rate {
                    chosen_acid = Some(acid_name.clone());
                    break;
                }
                roll -= acid.mutate_rate;
            }
            chosen_acid.unwrap()
        };
        gene.insert(insert_at, insert_value);
        gene
    }

    pub fn delete_mutate<R: Rng>(mut gene: Gene, rng: &mut R, conf: &conf::Conf) -> Gene {
        if !gene.is_empty() {
            let remove_at = rng.gen_range(0..gene.len());
            gene.remove(remove_at);
        }
        gene
    }

    pub fn new_species(&self, gene: Gene, stats: &mut Stats) -> Arc<Species> {
        let mut guard = self.species_list.lock().unwrap();
        let species = match guard.entry(gene) {
            Entry::Occupied(mut x) => {
                if let Some(y) = x.get().upgrade() {
                    return y;
                }
                let species = Arc::new(Species::new(x.key().clone(), &self.conf));
                *x.get_mut() = Arc::downgrade(&species);
                species
            }
            Entry::Vacant(x) => {
                let species = Arc::new(Species::new(x.key().clone(), &self.conf));
                x.insert(Arc::downgrade(&species));
                species
            }
        };
        *self.snapshot.lock().unwrap() = None;
        species
    }

    pub fn clone_species(&self, species: Arc<Species>, deamon: &mut Deamon) -> Arc<Species> {
        let mut rand = deamon.angelos.random.gen_range(0.0..1.0);
        let gene = if unlikely(rand <= self.conf.bio.mutation.insert) {
            SpeciesPool::insert_mutate(
                species.gene.clone(),
                &mut deamon.angelos.random,
                &*self.conf,
            )
        } else if unlikely({
            rand -= self.conf.bio.mutation.insert;
            rand <= self.conf.bio.mutation.remove
        }) {
            SpeciesPool::delete_mutate(
                species.gene.clone(),
                &mut deamon.angelos.random,
                &*self.conf,
            )
        } else {
            return species;
        };

        self.new_species(gene, deamon.angelos.stats())
    }
}

impl Species {
    pub fn new(gene: Gene, conf: &conf::Conf) -> Species {
        let mut stats = conf.bio.init.clone();
        for acid in gene.iter() {
            stats = stats + &conf.bio.acids[acid].prop;
        }
        let name = Species::name(&gene);
        Species {
            gene,
            name,
            wake_period: stats.watch_period.max(0.0) as Tick,
            wake_energy_consume: stats.wake_energy_consume.max(0.0) as EnergyT,
            energy_cost: stats.wake_energy_consume.max(1.0) as EnergyT,
            spawn_loss: stats.spawn_loss.max(0.0) as EnergyT,
            spawn_init_energy: stats.spawn_init_energy.max(1.0) as EnergyT,
            incubation_delay: stats.incubation_delay.max(0.0) as Tick,
            size: stats.size.map(|x| x.max(0.0) as Idx),
            watch_period: stats.watch_period.max(1.0) as AgeT,
            watch_range: stats.watch_area.max(1.0).sqrt() as u16,
            move_period: stats.move_period.max(1.0) as AgeT,
            move_cost: stats.move_cost.max(0.0) as EnergyT,
            eat_threshold: stats.eat_threshold.max(0.0) as EnergyT,
            eat_takes: stats.eat_takes.max(0.0) as EnergyT,

            max_hp: stats.combat.hit_point.max(1.0) as HitPointT,
            threat: stats.combat.threat as ThreatT,
            atk: ATK::Normal(stats.combat.atk.max(0.0) as HitPointT),
            attack_cost: stats.combat.atk_cost.max(0.0) as EnergyT,
            flee_threshold: stats.combat.flee_threshold as ThreatT,
            fight_back_threshold: stats.combat.fight_back_threshold as ThreatT,
            attack_range: Default::default(),
            chase_threshold: stats.combat.chase_threshold as ThreatT,
            regeneration: stats.combat.regeneration.max(0.0) as HitPointT,
            regeneration_cost: stats.combat.regeneration_cost.max(0.0) as HitPointT,
        }
    }

    pub fn name(gene: &Gene) -> String {
        gene.clone()
            .into_iter()
            .reduce(|x, y| x.add(y.as_str()))
            .unwrap_or_default()
    }
}

pub mod stat {
    use std::fmt::{Display, Formatter, Pointer};
    use std::hash::{Hash, Hasher};

    use super::*;

    pub struct SpeciesStat {
        p: Weak<Species>,
    }

    impl PartialEq for SpeciesStat {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            std::ptr::eq(self.p.as_ptr(), other.p.as_ptr())
        }
    }

    impl Eq for SpeciesStat {}

    impl Hash for SpeciesStat {
        #[inline]
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.p.as_ptr().hash(state)
        }
    }

    impl Display for SpeciesStat {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            if let Some(species) = self.p.upgrade() {
                write!(f, "{}", species.to_string())
            } else {
                write!(f, "<extinct>")
            }
        }
    }

    impl From<Weak<Species>> for SpeciesStat {
        fn from(p: Weak<Species>) -> Self {
            Self { p }
        }
    }

    impl From<&Arc<Species>> for SpeciesStat {
        fn from(a: &Arc<Species>) -> Self {
            Self {
                p: Arc::downgrade(a),
            }
        }
    }
}
