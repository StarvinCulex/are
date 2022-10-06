use std::ops::{Deref, Range};

use rand::Rng;

use crate::arena::gnd;
use crate::arena::types::EnvT;
use crate::math::noise::{another_perlin_donut, noise_unify};
use crate::mind::gen::gen_noise;
use crate::{math, Conf, Coord, Cosmos};

pub fn gen_terrain<RNG: Rng>(cosmos: &mut Cosmos, conf: &'_ Conf, rng: &mut RNG) {
    let stats = cosmos.angelos.stats.get_mut().unwrap();

    stats
        .benchmark
        .start_timing("generate terrain#env")
        .unwrap();

    // 设置湿度
    for (p, v) in gen_noise(rng, *cosmos.plate.size(), conf) {
        cosmos.plate[p].ground.env.wet = v;
    }
    // 设置高度
    // for (p, v) in gen_noise(rng, *cosmos.plate.size(), conf) {
    //     cosmos.plate[p].ground.env.altitude = v;
    // }

    stats.benchmark.stop_timing("generate terrain#env").unwrap();
}
