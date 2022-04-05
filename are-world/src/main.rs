#![feature(exclusive_range_pattern)]
#![feature(const_fn_trait_bound)]
#![feature(stmt_expr_attributes)]
#![feature(auto_traits)]
#![feature(negative_impls)]
#![feature(set_ptr_value)]
#![feature(get_mut_unchecked)]
#![feature(unsize)]
#![feature(coerce_unsized)]
// #![feature(dispatch_from_dyn)]
#![feature(scoped_threads)]
#![feature(let_chains)]

use crate::arena::conf::StaticConf;
use crate::arena::mind::gods::plant::GodOfPlant;
use crate::arena::mind::online::Gate;
use crate::arena::{RuntimeConf, P};
use crate::conencode::ConEncoder;
use crate::cui::Window;
use crate::grids::*;
use crate::sword::SWord;

mod arena;
mod conencode;
mod cui;
mod grids;
mod msgpip;
mod sword;

fn main() {
    let mut meta = arena::MetaCosmos::new(
        StaticConf {
            plate_size: Coord(10, 9),
        },
        RuntimeConf {
            period: 10,
            fire_tick: 4,
            plant_aging: 0.01,
            plant_sow: 0.0005,
        },
    );

    meta.cosmos
        .angelos
        .join(Box::new(Gate::listen("0.0.0.0:8964")));
    meta.cosmos.angelos.join(Box::new(GodOfPlant::new()));
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
