use std::ops::Range;
use std::sync::Arc;

use plotters::style::{RGBColor, BLACK};
use rand::rngs::StdRng;
use rand::Rng;

use crate::arena::gnd::Item;
use crate::arena::types::EnvT;
use crate::math::noise::{another_perlin_donut, noise_unify};
use crate::meta::gnd::plant;
use crate::mind::{gods, Mind};
use crate::{math, Block, Conf, Coord, Cosmos, Matrix, PKey, SeedableRng};

mod bio;
mod structures;
mod terrain;

pub struct Generator {
    rng: StdRng,
    conf: Arc<Conf>,
}

impl Generator {
    pub fn new(conf: Arc<Conf>) -> Self {
        Generator {
            rng: StdRng::from_entropy(),
            conf,
        }
    }
}

impl Mind for Generator {
    fn observe(&mut self, cosmos: &Cosmos, pk: &PKey) -> Result<(), ()> {
        Ok(())
    }

    fn make_move(&mut self, cosmos: &Cosmos, pk: &PKey) -> Result<(), ()> {
        Ok(())
    }

    fn set_cosmos(&mut self, cosmos: &mut Cosmos) -> Result<(), ()> {
        terrain::gen_terrain(cosmos, self.conf.as_ref(), &mut self.rng);

        structures::gen_structures(cosmos, self.conf.as_ref(), &mut self.rng);

        if let Err(e) = math::paint(
            &cosmos.plate,
            &self.conf.log.snapshot_paths.overview,
            &BLACK,
            terrain_painter,
        ) {
            //todo
        }

        bio::gen_bio(cosmos, &*self.conf, &mut self.rng);

        Err(())
    }

    fn name(&self) -> String {
        String::from("generator")
    }
}

pub fn gen_noise<RNG: Rng>(
    rng: &mut RNG,
    size: Coord<isize>,
    conf: &'_ Conf,
) -> Matrix<EnvT, 1, 1> {
    let size = size.map(|x| x as usize);
    const ENV_T_RANGE: Range<f32> = (EnvT::MIN as f32)..(EnvT::MAX as f32 + 1.0 - f32::EPSILON);
    let noise = another_perlin_donut(rng, size, conf.gen.unit_count, conf.gen.mesh_size);
    let noise = noise_unify(noise, ENV_T_RANGE);
    Matrix::with_iter(size, noise.into_iter().map(|(p, v)| (p, v as EnvT))).unwrap()
}

fn terrain_painter(grid: &Block) -> RGBColor {
    let base_color = RGBColor(0, 0, (grid.ground.env.wet as i32 + 128) as u8);

    base_color
}
