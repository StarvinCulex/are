use crate::mob::bio::species::SpeciesPool;
use crate::GameConf;

pub struct Singletons {
    pub species_pool: SpeciesPool,
}

impl Singletons {
    pub fn new(static_conf: &GameConf) -> Singletons {
        Singletons {
            species_pool: SpeciesPool::new(),
        }
    }
}
