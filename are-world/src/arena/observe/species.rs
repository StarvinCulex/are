use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::intrinsics::likely;

use serde::Serialize;

use crate::mob::bio::gene::Gene;
use crate::{if_likely, Cosmos, PKey, ReadGuard, Species};

#[derive(Serialize)]
pub struct SpeciesStats {
    pub species: HashMap<Gene, SpeciesView>,
}

#[derive(Serialize)]
pub struct SpeciesView {
    pub name: String,
    pub population: usize,
}

impl SpeciesStats {
    pub fn new(cosmos: &Cosmos, guard: &ReadGuard<PKey>) -> Self {
        let species = HashMap::from_iter(
            cosmos
                .angelos
                .singletons
                .species_pool
                .snapshot()
                .iter()
                .map(|(g, s)| {
                    (
                        g.clone(),
                        SpeciesView {
                            name: Species::name(g),
                            population: s.strong_count(),
                        },
                    )
                }),
        );

        SpeciesStats { species }
    }
}

impl Display for SpeciesView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{name}({pop})", name = self.name, pop = self.population)
    }
}

impl Display for SpeciesStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (gene, view) in self.species.iter() {
            writeln!(f, "{gene:?} : {view}", gene = gene, view = view)?;
        }
        Ok(())
    }
}
