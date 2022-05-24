#![feature(exclusive_range_pattern)]
#![feature(const_fn_trait_bound)]
#![feature(stmt_expr_attributes)]
#![feature(auto_traits)]
#![feature(negative_impls)]
#![feature(get_mut_unchecked)]
#![feature(unsize)]
#![feature(coerce_unsized)]
#![feature(dispatch_from_dyn)]
#![feature(is_some_with)]
#![feature(scoped_threads)]
#![feature(vec_into_raw_parts)]
#![feature(maybe_uninit_slice)]
#![feature(in_band_lifetimes)]
#![feature(ptr_metadata)]
#![feature(arbitrary_self_types)]
#![allow(dead_code, unused_imports, unused_variables)]

use std::ffi::{OsStr, OsString};
use std::io::Read;
use std::iter::Iterator;
use std::sync::Arc;

use crate::arena::conf::GameConf;
use crate::arena::cosmos::*;
use crate::arena::mind::gods::plant::GodOfPlant;
use crate::arena::mob::mech::mech::Mech;
use crate::arena::mob::Mob;
use crate::arena::r#ref::ReadGuard;
use crate::arena::RuntimeConf;
use crate::conencode::ConEncoder;
use crate::conf::Conf;
use crate::cui::Window;
use crate::grids::*;
use crate::mob::bio::bio::Bio;
use crate::mob::bio::species::Species;
use crate::sword::SWord;

// cargo update -p crossbeam-epoch:0.9.8 --precise 0.9.7

mod arena;
mod conencode;
mod cui;
mod grids;
mod jobs;
mod lock;
mod msgpip;
mod sword;

fn main() {
    let mut conf_path = None;
    {
        let mut args = std::env::args_os();
        while let Some(arg) = args.next() {
            if arg == *"-c" {
                conf_path = Some(args.next().unwrap_or_else(|| panic!("-c")));
            }
        }
    }

    let conf: Arc<Conf> = {
        let mut file = if let Some(cp) = conf_path {
            std::fs::File::open(cp).unwrap()
        } else {
            std::fs::File::open(
                std::env::current_exe()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join("are.toml")
                    .into_os_string(),
            )
            .or_else(|_| {
                std::fs::File::open(
                    std::env::current_dir()
                        .unwrap()
                        .join("are.toml")
                        .into_os_string(),
                )
            })
            .unwrap()
        };
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        Arc::new(toml::from_str(content.as_str()).unwrap())
    };

    println!("{:?}", conf);

    let mut meta = arena::MetaCosmos::new(conf.clone());

    // meta.cosmos
    //     .angelos
    //     .join(Box::new(Gate::listen("0.0.0.0:8964")));
    meta.cosmos
        .angelos
        .join(Box::new(GodOfPlant::new(conf.clone())));

    let adam = meta
        .cosmos
        .set(
            Bio::new(
                Arc::new(Species {
                    name: "".to_string(),
                }),
                50,
            )
            .into_box(),
        )
        .unwrap_or_else(|_| panic!());
    let mut worker = meta.cosmos.angelos.make_worker();
    worker.tell(adam, mob::Msg::Wake, 1);
    drop(worker);

    // meta.cosmos.plate[Coord(0isize, 0)].mob = Some(Mech {}.into_block());
    // meta.cosmos.angelos.order(Coord(0, 0), mob::Order::Wake, 1);

    meta.step();
    loop {
        let mut mobs = vec![];
        println!("=====");
        println!(
            "{}",
            meta.cosmos.plate.as_area().map(|b| {
                if let Some(mob) = b.mob() {
                    mobs.push((mob.0, mob.1.clone()));
                    "mob".to_string()
                } else {
                    b.ground.name()
                }
            })
        );
        meta.cosmos.pk(|cosmos, pkey| {
            for (p, mob) in mobs.iter() {
                println!(
                    "mob {} {}",
                    p,
                    ReadGuard::with(pkey, |guard| {
                        if let Some(mob) = guard.wrap_weak(mob.clone()) {
                            mob.to_string()
                        } else {
                            "".to_string()
                        }
                    })
                );
            }
        });
        meta.step();
        std::thread::sleep(std::time::Duration::from_millis(conf.runtime.period));
    }
}
