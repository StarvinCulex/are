use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::{write, Display, Formatter};
use std::intrinsics::{likely, unlikely};

use serde::Serialize;

use crate::meta::defs::CrdI;
use crate::meta::types::EnergyT;
use crate::{
    if_likely, if_unlikely, Bio, Block, Coord, Cosmos, Interval, Matrix, Mob, MobRef, PKey,
    ReadGuard,
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
    pub item: String,
    pub energy: EnergyT,
}

#[derive(Serialize)]
pub struct MobView {
    pub name: String,
    pub properties: String,
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
                .mob_ref(guard)
                .map(|m| {
                    // make it shorter
                    let ptr = &*m as *const dyn Mob as *const () as usize;
                    if let Entry::Vacant(entry) = mobs.entry(ptr) {
                        entry.insert(MobView::new(m));
                    }
                    ptr
                })
                .unwrap_or_default(),
            item: block.ground.item.to_string(),
            energy: block.ground.energy(),
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
        let at = mob.at();
        MobView {
            name: mob.get_name(),
            properties: mob
                .downcast()
                .map(|m: MobRef<Bio>| {
                    let g = m.target.lock().unwrap();
                    let target = &g.target;
                    format!(
                        "{at} H{hp} A{age} E{energy} S{species} T{{target{target} action{action:?} range{action_range} tmob{target_mob:x}}}",
                        at = at,
                        hp = m.hp,
                        age = m.age,
                        energy = m.energy,
                        species = m.species.name,
                        target = target.target.map_or("-".to_string(), |x| x.to_string()),
                        action = target.action,
                        action_range = target.action_range,
                        target_mob = target.target_mob.as_ref().map_or(0usize, |x| x.as_ptr() as *const () as usize),
                    )
                })
                .unwrap_or_else(|_| "-".to_string()),
        }
    }
}

impl Display for BlockView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.mob == 0 {
            write!(f, "{}{}", self.item, self.energy)
        } else {
            write!(f, "[{:x}]({}{})", self.mob, self.item, self.energy)
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
            writeln!(f, "{:x} {}", id, mob.properties)?;
        }
        Ok(())
    }
}
