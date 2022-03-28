use std::sync::Arc;

use crate::arena::body::bio::gene::Gene;
use crate::SWord;

pub struct Species {
    pub name: SWord,
    pub gene: Gene,
}

pub struct SpeciesPool {}
