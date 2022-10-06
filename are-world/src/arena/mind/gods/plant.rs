use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::sync::Arc;

use crate::arena::gnd::plant;
use crate::arena::mind::Mind;
use crate::arena::{gnd, Cosmos, Orderer};
use crate::meta::gnd::plant::Kind;
use crate::meta::gnd::Ground;
use crate::meta::types;
use crate::{conf, Coord, PKey};

pub struct GodOfPlant {
    pub conf: Arc<conf::Conf>,

    rng: StdRng,
}

impl GodOfPlant {
    pub fn new(conf: Arc<conf::Conf>) -> GodOfPlant {
        GodOfPlant {
            conf,
            rng: StdRng::from_entropy(),
        }
    }
}

impl Mind for GodOfPlant {
    fn observe(&mut self, cosmos: &Cosmos, pk: &PKey) -> Result<(), ()> {
        Ok(())
    }

    fn make_move(&mut self, cosmos: &Cosmos, pk: &PKey) -> Result<(), ()> {
        Ok(())
    }

    fn set_cosmos(&mut self, cosmos: &mut Cosmos) -> Result<(), ()> {
        Ok(())
    }

    fn name(&self) -> String {
        String::from("plant of god")
    }
}
