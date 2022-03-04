#![feature(exclusive_range_pattern)]
#![feature(const_fn_trait_bound)]
#![feature(adt_const_params)]

use crate::arena::conf::StaticConf;
use crate::arena::mind::online::Gate;
use crate::arena::RuntimeConf;
use crate::conencode::ConEncoder;
use crate::cui::Window;
use crate::grids::*;
use crate::sword::SWord;

mod arena;
mod conencode;
mod cui;
mod grids;
mod sword;

fn main() {
    let mut meta = arena::MetaCosmos::new(
        StaticConf {
            plate_size: Coord(10, 9),
        },
        RuntimeConf {
            period: 1000,
            fire_tick: 3,
        },
    );
    meta.cosmos.plate[Coord(1isize, 1isize)].body.name = SWord::new("hi");
    meta.cosmos
        .angelos
        .join(Box::new(Gate::listen("0.0.0.0:8964")));
    // 滑翔机？
    meta.cosmos.burn(Coord(1isize, 6isize));
    meta.cosmos.burn(Coord(2isize, 6isize));
    meta.cosmos.burn(Coord(3isize, 6isize));
    meta.cosmos.burn(Coord(3isize, 7isize));
    meta.cosmos.burn(Coord(2isize, 8isize));
    loop {
        println!("=====");
        println!("{}", meta.cosmos.plate.as_area().map(|b| b.body.name));
        meta.step();
        std::thread::sleep(std::time::Duration::from_millis(
            meta.cosmos.angelos.properties.runtime_conf.period,
        ));
    }
}
