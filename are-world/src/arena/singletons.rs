use crate::mob::bio::species::SpeciesPool;
use crate::StaticConf;

pub struct Singletons {
    pub species_pool: SpeciesPool,
}

impl Singletons {
    pub fn new(static_conf: &StaticConf) -> Singletons {
        Singletons {
            species_pool: SpeciesPool::new(),
        }
    }
}
