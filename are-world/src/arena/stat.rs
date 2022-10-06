use std::fmt::{Debug, Display, Formatter};

use derive_more::{Add, AddAssign};

use crate::mob::bio::species::stat::SpeciesStat;
use crate::stats;
use crate::stats::bm2;

#[derive(Default, Debug, Add, AddAssign)]
pub struct Stats {
    pub bio_energy_store: stats::chart::Chart<SpeciesStat, stats::chart::Count>,

    pub benchmark: bm2::Benchmark,
}

impl Stats {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
