use rand::Rng;

use crate::arena::gnd;
use crate::{Conf, Cosmos};

pub fn gen_structures<RNG: Rng>(cosmos: &mut Cosmos, conf: &'_ Conf, rng: &mut RNG) {
    crate::benchmark_time!(
        ["generate structures#plant", &mut cosmos.angelos.stats.get_mut().unwrap().benchmark]
        // 放置植物
        for (pos, block) in cosmos.plate.as_area_mut() {
            for id in 0..gnd::plant::prop::DETAIL.len() {
                let detail = gnd::plant::prop::DETAIL[id];
                let id = gnd::plant::Kind::try_from(id).unwrap();
                let p = detail.gen_possibility(&block.ground.env);
                if !(0.0..1.0 + f32::EPSILON).contains(&p) {
                    panic!()
                }

                if rng.gen_bool(p as f64) {
                    let p = gnd::Plant::new(id, &cosmos.angelos);

                    block.ground.item = gnd::Item::Plant(p);
                    break;
                }
            }
        }
    );
}
