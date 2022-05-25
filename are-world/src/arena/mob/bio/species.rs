use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ops::Add;
use std::sync;
use std::sync::{Arc, Weak};

use rand::distributions::Uniform;
use rand::Rng;

use crate::arena::conf;
use crate::arena::defs::Crd;
use crate::arena::types::*;
use crate::conf::bio::Acid;
use crate::lock::spinlock::SpinLock;
use crate::meta::defs::{Idx, Tick};
use crate::mob::bio::bio::{BioAction, BioTarget};
use crate::{Block, Coord, Deamon};

use super::gene::Gene;

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
}

pub struct SpeciesPool {
    conf: Arc<conf::Conf>,
    species_list: SpinLock<HashMap<Gene, Weak<Species>>>,
}

impl Species {
    pub fn bio_count(species: Weak<Species>) -> usize {
        species.strong_count()
    }

    /// 返回值：负数表示逃离，正数表示接近
    pub fn watching_choice(&self, at: Crd, block: &Block) -> BioTarget {
        if block.ground.plant.age >= self.eat_threshold {
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
            conf,
            species_list: SpinLock::default(),
        }
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

    pub fn new_species(&self, gene: Gene) -> Arc<Species> {
        let mut guard = self.species_list.lock().unwrap();
        match guard.entry(gene) {
            Entry::Occupied(mut x) => {
                if let Some(y) = x.get().upgrade() {
                    return y;
                }
                let species = Arc::new(Species::new(x.key().clone(), &self.conf));
                *x.get_mut() = std::sync::Arc::downgrade(&species);
                species
            }
            Entry::Vacant(x) => {
                let species = Arc::new(Species::new(x.key().clone(), &self.conf));
                x.insert(std::sync::Arc::downgrade(&species));
                species
            }
        }
    }

    pub fn clone_species(&self, species: sync::Arc<Species>, deamon: &mut Deamon) -> Arc<Species> {
        let mut rand = deamon.angelos.random.gen_range(0.0..1.0);
        let gene = if rand <= self.conf.bio.mutation.insert {
            Some(SpeciesPool::insert_mutate(
                species.gene.clone(),
                &mut deamon.angelos.random,
                &*self.conf,
            ))
        } else if {
            rand -= self.conf.bio.mutation.insert;
            rand <= self.conf.bio.mutation.remove
        } {
            Some(SpeciesPool::delete_mutate(
                species.gene.clone(),
                &mut deamon.angelos.random,
                &*self.conf,
            ))
        } else {
            None
        };

        if let Some(new_gene) = gene {
            self.new_species(new_gene)
        } else {
            species
        }
    }
}

impl Species {
    fn new(gene: Gene, conf: &conf::Conf) -> Species {
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
            watch_range: stats.watch_range.max(0.0) as u16,
            move_period: stats.move_period.max(0.0) as AgeT,
            move_cost: stats.move_cost.max(0.0) as EnergyT,
            eat_threshold: stats.eat_threshold.max(0.0) as EnergyT,
            eat_takes: stats.eat_takes.max(0.0) as EnergyT,
        }
    }

    fn name(gene: &Gene) -> String {
        gene.clone()
            .into_iter()
            .reduce(|x, y| x.add(y.as_str()))
            .unwrap_or_default()
    }
}
