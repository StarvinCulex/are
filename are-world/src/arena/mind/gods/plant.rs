use std::borrow::{Borrow, BorrowMut};

use rand::distributions::Distribution;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::arena::gnd::plant;
use crate::arena::mind::Mind;
use crate::arena::{gnd, Cosmos, Orderer};
use crate::Coord;

pub struct GodOfPlant {
    rng: rand::rngs::StdRng,
}

impl GodOfPlant {
    const KIND_LIST: [plant::Kind; 2] = [plant::Kind::Grass, plant::Kind::Tree];

    pub fn new() -> GodOfPlant {
        GodOfPlant {
            // how to get seed?
            rng: rand::rngs::StdRng::from_seed([0; 32]),
        }
    }
}

impl Mind for GodOfPlant {
    fn observe(&mut self, cosmos: &Cosmos) -> Result<(), ()> {
        Ok(())
    }

    fn make_move(&mut self, cosmos: &Cosmos) -> Result<(), ()> {
        let plate_distributes = cosmos
            .plate
            .size()
            .map(|x| rand::distributions::Uniform::from(0..x));
        let area = cosmos.plate.size().0 * cosmos.plate.size().1;

        let aging_count = cosmos.angelos.properties.runtime_conf.plant_aging * area as f64;
        if aging_count >= 1.0 || {
            let aging_distributes = rand::distributions::Uniform::from(0.0..1.0);
            let p = self.rng.sample(aging_distributes);
            p <= aging_count
        } {
            for _ in 0..aging_count as usize {
                let p = Coord(
                    self.rng.sample(plate_distributes.0),
                    self.rng.sample(plate_distributes.1),
                );
                cosmos
                    .angelos
                    .order(p.try_into().unwrap(), gnd::Order::PlantAging, 0);
            }
        }

        let sow_count = (cosmos.angelos.properties.runtime_conf.plant_sow * area as f64).ceil();
        if sow_count >= 1.0 || {
            let sow_distributes = rand::distributions::Uniform::from(0.0..1.0);
            let p = self.rng.sample(sow_distributes);
            p <= sow_count
        } {
            let kind_distributes =
                rand::distributions::Uniform::from(0..GodOfPlant::KIND_LIST.len());
            for _ in 0..sow_count as usize {
                let p = Coord(
                    self.rng.sample(plate_distributes.0),
                    self.rng.sample(plate_distributes.1),
                );
                let kind = GodOfPlant::KIND_LIST[self.rng.sample(kind_distributes)];
                cosmos
                    .angelos
                    .order(p.try_into().unwrap(), gnd::Order::PlantSowing(kind), 0);
            }
        }

        Ok(())
    }

    fn set_cosmos(&mut self, cosmos: &mut Cosmos) -> Result<(), ()> {
        Ok(())
    }
}
