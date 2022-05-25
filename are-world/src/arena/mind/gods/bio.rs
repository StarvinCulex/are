use std::sync::Arc;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::mind::Mind;
use crate::mob::bio::species::SpeciesPool;
use crate::{conf, mob, Bio, Coord, Cosmos, Mob, PKey, Species, Teller};

pub struct GodOfBio {
    pub conf: Arc<conf::Conf>,
    rng: StdRng,
}

impl GodOfBio {
    pub fn new(conf: Arc<conf::Conf>) -> GodOfBio {
        GodOfBio {
            conf,
            rng: StdRng::from_entropy(),
        }
    }
}

impl Mind for GodOfBio {
    fn observe(&mut self, cosmos: &Cosmos, pk: &PKey) -> Result<(), ()> {
        Ok(())
    }

    fn make_move(&mut self, cosmos: &Cosmos, pk: &PKey) -> Result<(), ()> {
        Ok(())
    }

    fn set_cosmos(&mut self, cosmos: &mut Cosmos) -> Result<(), ()> {
        {
            let p = self.rng.gen_range(0.0..1.0);
            if p < self.conf.bio.create_possibility {
                for _ in 0..(self.conf.bio.create_possibility.ceil() as usize) {
                    let gene = {
                        let mut gene = None;
                        let mut creature = None;
                        let mut roll = self
                            .rng
                            .gen_range(0..self.conf.bio.creatures.iter().map(|x| x.weight).sum());
                        for c in self.conf.bio.creatures.iter() {
                            if roll < c.weight {
                                gene = Some(c.gene.clone());
                                creature = Some(c);
                                break;
                            }
                            roll -= c.weight;
                        }
                        for _ in 0..creature.unwrap().insertions {
                            gene = Some(SpeciesPool::insert_mutate(
                                gene.unwrap(),
                                &mut self.rng,
                                &*self.conf,
                            ));
                        }
                        gene.unwrap()
                    };

                    let species = cosmos.angelos.singletons.species_pool.new_species(gene);
                    let init_energy = species.spawn_init_energy;
                    let init_size = species.size;
                    let wake_delay = species.incubation_delay;
                    let mut mob = Bio::new(species, init_energy).into_box();

                    for _ in 0..self.conf.runtime.retry {
                        let at = (Coord(0, 0) | init_size).offset(Coord(
                            self.rng.gen_range(0..cosmos.angelos.plate_size.0),
                            self.rng.gen_range(0..cosmos.angelos.plate_size.1),
                        ));
                        mob.at = at;
                        match cosmos.set(mob) {
                            Err(m) => mob = m,
                            Ok(m) => {
                                cosmos
                                    .angelos
                                    .make_worker()
                                    .tell(m, mob::Msg::Wake, wake_delay);
                                break;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
