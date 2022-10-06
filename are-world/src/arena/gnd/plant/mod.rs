use std::fmt::Debug;
use std::intrinsics::{likely, unlikely};
use std::num::NonZeroU8;
use std::ops::{AddAssign, DivAssign};
use std::sync::atomic::{AtomicU8, Ordering};

use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::arena::cosmos::Deamon;
use crate::arena::defs::Crd;
use crate::arena::gnd::plant::prop::PlantClass;
use crate::arena::types::*;
use crate::arena::{gnd, Cosmos, Orderer};
use crate::meta::defs::Tick;
use crate::{conf, if_likely, if_unlikely, Coord, MajorAngelos};

use super::Environment;

pub mod prop;

#[derive(Serialize, Deserialize, Clone)]
pub struct Plant {
    pub kind: Kind,
    pub birthday: Tick,
}

pub type Kind = u8;

impl Plant {
    #[inline]
    pub fn new(kind: Kind, angelos: &MajorAngelos) -> Self {
        let birthday = angelos.properties.tick;
        Plant { kind, birthday }
    }
    #[inline]
    pub fn kind_detail(&self) -> &'static dyn PlantClass {
        prop::DETAIL[self.kind as usize]
    }
}

impl ToString for Plant {
    #[inline]
    fn to_string(&self) -> String {
        prop::DETAIL[usize::from(self.kind)].to_string()
    }
}
