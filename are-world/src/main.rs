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
#![feature(core_intrinsics)]

use std::ffi::{OsStr, OsString};
use std::io::Read;
use std::iter::Iterator;
use std::sync::Arc;
use std::time::SystemTime;

use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

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
use crate::mind::gods::bio::GodOfBio;
use crate::mob::bio::bio::Bio;
use crate::mob::bio::species::Species;
use crate::stats::benchmark::Benchmark;
use crate::sword::SWord;

// cargo update -p crossbeam-epoch:0.9.8 --precise 0.9.7

mod arena;
mod conencode;
mod cui;
mod grids;
mod jobs;
mod likely;
mod lock;
mod msgpip;
mod stats;
mod sword;

// #[inline(never)]
// fn benchmark_sub<const CW: usize, const CH: usize>(size: Coord<usize>) -> Matrix<String, CW, CH> {
//     Matrix::<_, CW, CH>::with_ctor(size, |p| p.to_string())
// }
// #[inline(never)]
// fn benchmark<const CW: usize, const CH: usize>(
//     size: Coord<usize>,
//     offset: Coord<isize>,
// ) -> (Matrix<String, CW, CH>, usize, usize) {
//     let mut benchmark = Benchmark::new();
//     benchmark.start();
//     let mut mat = benchmark_sub::<CW, CH>(size);
//     benchmark.end(format! {"init CW{} CH{} size{} offset{}", CW, CH, size, offset});
//     let mut rng = StdRng::from_seed([1; 32]);
//     let unix = Uniform::new(0, size.0 as isize);
//     let uniy = Uniform::new(0, size.1 as isize);
//     let cnt = 100_0000;
//     let area = Coord(0, 0) | offset;
//     benchmark.end("-");
//
//     let mut fuck = 0usize;
//     println!("{}", 0);
//     for _ in 0..cnt {
//         let p = Coord(rng.sample(unix), rng.sample(uniy));
//         for (p, v) in mat.area(area.offset(p)).scan() {
//             fuck = fuck
//                 .overflowing_add(v.as_str() as *const str as *const () as usize)
//                 .0;
//         }
//     }
//     benchmark.end(format!(
//         "read CW{} CH{} size{} offset{}",
//         CW, CH, size, offset
//     ));
//
//     let mut a = 0usize;
//     println!("{}", 1);
//     for _ in 0..100000 {
//         let p = Coord(rng.sample(unix), rng.sample(uniy));
//         for (p, v) in mat.area_mut(area.offset(p)).scan() {
//             *v = a.to_string();
//             a += 1;
//         }
//     }
//     benchmark.end(format!(
//         "write CW{} CH{} size{} offset{}",
//         CW, CH, size, offset
//     ));
//     println!("{}", benchmark);
//
//     (mat, fuck, a)
// }
//
// fn main() {
//     benchmark::<1, 1>(Coord(100000, 500), Coord(100, 100));
//     benchmark::<8, 8>(Coord(100000, 500), Coord(100, 100));
//     benchmark::<4, 4>(Coord(100000, 500), Coord(100, 100));
//     benchmark::<1, 16>(Coord(100000, 500), Coord(100, 100));
//     benchmark::<16, 1>(Coord(100000, 500), Coord(100, 100));
// }

fn main() {
    let mut conf_path = None;
    {
        let mut args = std::env::args_os();
        while let Some(arg) = args.next() {
            if arg == *"-c" {
                conf_path = Some(args.next().unwrap_or_else(|| panic!("-c")));
            } else {
                println!("unexpected param `{}`", arg.into_string().unwrap());
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

    let mut meta = MetaCosmos::new(conf.clone());

    meta.cosmos
        .angelos
        .join(Box::new(GodOfPlant::new(conf.clone())));
    meta.cosmos
        .angelos
        .join(Box::new(GodOfBio::new(conf.clone())));

    loop {
        let start = SystemTime::now();
        for i in 0..1 {
            meta.step();
            if conf.runtime.period != 0 {
                std::thread::sleep(std::time::Duration::from_millis(conf.runtime.period));
            }
        }
        let duration = SystemTime::now().duration_since(start).unwrap();
        println!("cost {}ms", duration.as_millis());
        println!("benchmarks\n{}", meta.benchmark);
        meta.benchmark.clear();
        meta.cosmos.pk(|cosmos, pkey| {
            let area = Coord(0, 0) | (*cosmos.plate.size() - Coord(1, 1));
            ReadGuard::with(pkey, |guard| {
                // let view = observe::species::SpeciesStats::new(cosmos, guard);
                // println!("{}", view);
                println!(
                    "{}",
                    observe::plate::PlateView::new(
                        cosmos,
                        Coord(0isize, 0) | Coord(20isize, 20),
                        guard,
                    )
                );
            });
        });
    }
}
