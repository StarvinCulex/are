use std::sync::Arc;

use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::arena::gnd::plant;
use crate::arena::mind::Mind;
use crate::arena::{gnd, Cosmos, Orderer};
use crate::meta::gnd::plant::Kind;
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
        let mut angelos = cosmos.angelos.make_worker();

        let plate_distributes = cosmos.angelos.plate_size.map(|x| Uniform::from(0..x));
        let area = cosmos.plate.size().0 * cosmos.plate.size().1;

        let aging_count = self.conf.plant.aging.possibility * area as f64;
        if aging_count >= 1.0 || {
            let aging_distributes = Uniform::from(0.0..1.0);
            let p = self.rng.sample(aging_distributes);
            p <= aging_count
        } {
            for _ in 0..(aging_count.ceil()) as usize {
                let p = Coord(
                    self.rng.sample(plate_distributes.0),
                    self.rng.sample(plate_distributes.1),
                );
                angelos.order(p, gnd::Order::PlantAging, 0);
            }
        }

        let sow_count = self.conf.plant.sow.possibility * area as f64;
        if sow_count >= 1.0 || {
            let sow_distributes = Uniform::from(0.0..1.0);
            let p = self.rng.sample(sow_distributes);
            p <= sow_count
        } {
            for _ in 0..(sow_count.ceil() as usize) {
                let p = Coord(
                    self.rng.sample(plate_distributes.0),
                    self.rng.sample(plate_distributes.1),
                );
                angelos.order(
                    p,
                    gnd::Order::PlantSowing(Kind::random_new(&self.conf.plant, &mut self.rng)),
                    0,
                );
            }
        }

        Ok(())
    }

    fn set_cosmos(&mut self, cosmos: &mut Cosmos) -> Result<(), ()> {
        Ok(())
    }
}
