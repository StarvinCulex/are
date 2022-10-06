use rand::Rng;

use crate::mob::bio::species::SpeciesPool;
use crate::{mob, Bio, Conf, Coord, Cosmos, Mob, Teller};

pub fn gen_bio<RNG: Rng>(cosmos: &mut Cosmos, conf: &'_ Conf, rng: &mut RNG) {
    cosmos
        .angelos
        .stats
        .get_mut()
        .unwrap()
        .benchmark
        .start_timing("generate bio")
        .unwrap();

    for _ in 0..conf.bio.initial_population {
        let gene = {
            let mut gene = None;
            let mut creature = None;
            let mut roll = rng.gen_range(0..conf.bio.creatures.iter().map(|x| x.weight).sum());
            for c in conf.bio.creatures.iter() {
                if roll < c.weight {
                    gene = Some(c.gene.clone());
                    creature = Some(c);
                    break;
                }
                roll -= c.weight;
            }
            for _ in 0..creature.unwrap().insertions {
                gene = Some(SpeciesPool::insert_mutate(gene.unwrap(), rng, conf));
            }
            gene.unwrap()
        };

        let species = cosmos
            .angelos
            .singletons
            .species_pool
            .new_species(gene, &mut *cosmos.angelos.stats.lock().unwrap());
        let init_energy = species.spawn_init_energy;
        let init_size = species.size;
        let wake_delay = species.incubation_delay;
        let mut mob = Bio::new(species, &mut cosmos.angelos, init_energy).into_box();

        for _ in 0..conf.runtime.retry {
            let at = (Coord(0, 0) | init_size).offset(Coord(
                rng.gen_range(0..cosmos.angelos.plate_size.0),
                rng.gen_range(0..cosmos.angelos.plate_size.1),
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

    cosmos
        .angelos
        .stats
        .get_mut()
        .unwrap()
        .benchmark
        .stop_timing("generate bio")
        .unwrap();
}
