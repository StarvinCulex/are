use std::ops::{Deref, Range};

use rand::Rng;

use crate::arena::gnd;
use crate::arena::types::EnvT;
use crate::math::noise::{another_perlin_donut, noise_unify};
use crate::mind::gen::gen_noise;
use crate::{math, Conf, Coord, Cosmos};

pub fn gen_terrain<RNG: Rng>(cosmos: &mut Cosmos, conf: &'_ Conf, rng: &mut RNG) {
    // 设置湿度
    for (p, v) in gen_noise(rng, *cosmos.plate.size(), conf) {
        cosmos.plate[p].ground.env.humid = v;
    }
    // 设置高度
    for (p, v) in gen_noise(rng, *cosmos.plate.size(), conf) {
        cosmos.plate[p].ground.env.altitude = v;
    }

    // 放置植物
    for (pos, block) in cosmos.plate.as_area_mut() {
        for id in 0..gnd::plant::prop::DETAIL.len() {
            let detail = &gnd::plant::prop::DETAIL[id];
            let id = gnd::plant::Kind::try_from(id).unwrap();
            let p = detail.gen.possibility(&block.ground.env);
            if !(0.0..1.0).contains(&p) {
                panic!()
            }

            if rng.gen_bool(p as f64) {
                let mut p = gnd::Plant::new(id);
                p.energy = rng.gen_range(0..detail.max_energy);

                block.ground.item = gnd::Item::Plant(p);
                break;
            }
        }
    }
}
