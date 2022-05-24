use std::sync::Arc;

use crate::mob::bio::species::SpeciesPool;
use crate::{conf, GameConf};

pub struct Singletons {
    pub species_pool: SpeciesPool,
}

impl Singletons {
    pub fn new(conf: Arc<conf::Conf>) -> Singletons {
        Singletons {
            species_pool: SpeciesPool::new(),
        }
    }
}
