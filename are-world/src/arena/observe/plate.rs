use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::{write, Display, Formatter};
use std::intrinsics::{likely, unlikely};

use serde::Serialize;

use crate::meta::defs::CrdI;
use crate::meta::types::EnergyT;
use crate::{
    if_likely, if_unlikely, Block, Coord, Cosmos, Interval, Matrix, Mob, MobRef, PKey, ReadGuard,
};

#[derive(Serialize)]
pub struct PlateView {
    pub area: Coord<Interval<isize>>,
    pub matrix: Matrix<BlockView, 1, 1>,
    pub mobs: HashMap<usize, MobView>,
}

#[derive(Serialize)]
pub struct BlockView {
    pub mob: usize,
    pub plant_type: &'static str,
    pub plant_age: EnergyT,
}

#[derive(Serialize)]
pub struct MobView {
    pub name: String,
}

impl PlateView {
    pub fn new<Index: Into<isize> + Ord + Copy>(
        cosmos: &Cosmos,
        area: Coord<Interval<Index>>,
        guard: &ReadGuard<PKey>,
    ) -> PlateView {
        let mut mobs = HashMap::new();

        let matrix = cosmos.plate.area(area).map(|block| BlockView {
            mob: block
                .mob()
                .and_then(|(_, m)| guard.wrap_weak(m))
                .map(|m| {
                    // make it shorter
                    let ptr = &*m as *const dyn Mob as *const () as usize;
                    if let Entry::Vacant(entry) = mobs.entry(ptr) {
                        entry.insert(MobView::new(m));
                    }
                    ptr
                })
                .unwrap_or_default(),
            plant_type: block.ground.plant.kind.as_str(),
            plant_age: block.ground.plant.age,
        });
        PlateView {
            area: Coord(
                Interval::new(area.0.from.into(), area.0.to.into()),
                Interval::new(area.1.from.into(), area.1.to.into()),
            ),
            matrix,
            mobs,
        }
    }
}

impl MobView {
    fn new(mob: MobRef<dyn Mob>) -> Self {
        MobView {
            name: mob.get_name(),
        }
    }
}

impl Display for BlockView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.mob == 0 {
            write!(f, "{}", self.plant_type)
        } else {
            write!(f, "[{:x}]", self.mob)
        }
    }
}

impl Display for MobView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Display for PlateView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.matrix)?;
        for (id, mob) in self.mobs.iter() {
            writeln!(f, "{:x} {}", id, mob)?;
        }
        Ok(())
    }
}
