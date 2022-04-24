use crate::arena;
use crate::arena::types::EnergyT;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Order {
    PlantAging,
    PlantSowing(arena::gnd::plant::Kind),
    PlantMow(EnergyT),
}
