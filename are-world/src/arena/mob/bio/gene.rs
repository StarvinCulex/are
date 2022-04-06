use crate::arena::cosmos;

use super::atk::ATK;
use super::types::*;

pub struct Gene {}

pub enum Acid {
    A,
    T,
    G,
    C,
}

impl Gene {
    #[inline]
    pub fn max_hp(&self) -> HealthT {
        todo!()
    }
    #[inline]
    pub fn heartbeat(&self) -> HeartbeatT {
        todo!()
    }
    #[inline]
    pub fn atk(&self) -> ATK {
        todo!()
    }
    #[inline]
    pub fn dmg(&self, atk: ATK) -> HealthT {
        todo!()
    }
    #[inline]
    pub fn select(&self, block: cosmos::Block) -> i32 {
        todo!()
    }
    #[inline]
    pub fn speed(&self) -> SpeedT {
        todo!()
    }
}
