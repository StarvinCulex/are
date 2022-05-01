#![feature(exclusive_range_pattern)]
#![feature(const_fn_trait_bound)]
#![feature(stmt_expr_attributes)]
#![feature(auto_traits)]
#![feature(negative_impls)]
#![feature(get_mut_unchecked)]
#![feature(unsize)]
#![feature(coerce_unsized)]
#![feature(is_some_with)]
#![feature(scoped_threads)]
#![feature(vec_into_raw_parts)]
#![feature(maybe_uninit_slice)]

use crate::arena::conf::StaticConf;
use crate::arena::cosmos::*;
use crate::arena::mind::gods::plant::GodOfPlant;
use crate::arena::mind::online::Gate;
use crate::arena::mob::mech::mech::Mech;
use crate::arena::mob::Mob;
use crate::arena::{RuntimeConf, P};
use crate::conencode::ConEncoder;
use crate::cui::Window;
use crate::grids::*;
use crate::sword::SWord;

// cargo update -p crossbeam-epoch:0.9.8 --precise 0.9.7

mod arena;
mod conencode;
mod cui;
mod grids;
mod jobs;
mod msgpip;
mod sword;

fn main() {
    let mut meta = arena::MetaCosmos::new(
        StaticConf {
            chunk_count: Coord(10, 9),
            chunk_size: Coord(1024, 1024),
        },
        RuntimeConf {
            period: 10,
            fire_tick: 4,
            plant_aging: 0.1,
            plant_sow: 0.05,
            thread_count: 1,
        },
    );

    // meta.cosmos
    //     .angelos
    //     .join(Box::new(Gate::listen("0.0.0.0:8964")));
    meta.cosmos.angelos.join(Box::new(GodOfPlant::new()));

    // meta.cosmos.plate[Coord(0isize, 0)].mob = Some(Mech {}.into_block());
    // meta.cosmos.angelos.order(Coord(0, 0), mob::Order::Wake, 1);

    meta.step();
    loop {
        println!("=====");
        println!("{}", meta.cosmos.plate.as_area().map(|b| b.ground.name()));
        meta.step();
        std::thread::sleep(std::time::Duration::from_millis(
            meta.cosmos.angelos.properties.runtime_conf.period,
        ));

        meta.cosmos.angelos.properties.runtime_conf.plant_sow = 0.0;
    }
}
