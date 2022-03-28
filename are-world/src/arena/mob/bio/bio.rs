use std::sync::atomic::{AtomicU16, AtomicU8};
use std::sync::Arc;

use crate::arena::body::bio::atk::ATK;
use crate::arena::body::bio::species::Species;

pub struct Bio {
    pub species: Arc<Species>,
}

pub struct Status {}
