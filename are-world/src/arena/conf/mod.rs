use serde::Deserialize;

use crate::arena::defs::Crd;
use crate::meta::types::*;
use crate::Coord;

#[derive(Deserialize, Debug)]
pub struct Conf {
    pub game: game::Conf,
    pub runtime: runtime::Conf,
    pub log: log::Conf,

    pub plant: plant::Conf,
    pub bio: bio::Conf,
    pub gen: gen::Conf,
    pub corpse: corpse::Conf,
}

pub mod bio;
pub mod corpse;
pub mod game;
pub mod gen;
pub mod log;
pub mod plant;
pub mod runtime;
