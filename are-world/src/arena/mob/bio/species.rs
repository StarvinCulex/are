use std::sync;
use std::sync::Arc;

use crate::arena::conf;

use super::gene::Gene;

pub struct Species {
    pub name: String,
    pub gene: Gene,
}

pub struct SpeciesPool {}

impl SpeciesPool {
    pub fn new(conf: &conf::RuntimeConf) -> SpeciesPool {
        todo!()
    }

    pub fn clone_species(species: sync::Weak<Species>) -> Arc<Species> {
        todo!()
    }
}
