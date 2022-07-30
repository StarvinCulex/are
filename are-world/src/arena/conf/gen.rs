use serde::Deserialize;

use crate::Coord;

#[derive(Deserialize, Debug)]
pub struct Conf {
    pub unit_count: Coord<usize>,
    pub mesh_size: u8,
}
