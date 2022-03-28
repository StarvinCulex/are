use crate::arena;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Order {
    PlantAging,
    PlantSowing(arena::gnd::plant::Kind),
}
