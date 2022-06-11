#![feature(exclusive_range_pattern)]
#![feature(const_fn_trait_bound)]
#![feature(auto_traits)]
#![feature(negative_impls)]
#![feature(get_mut_unchecked)]
#![feature(unsize)]
#![feature(coerce_unsized)]
#![feature(dispatch_from_dyn)]
#![feature(is_some_with)]
#![feature(scoped_threads)]
#![feature(vec_into_raw_parts)]
#![feature(in_band_lifetimes)]
#![feature(arbitrary_self_types)]
#![feature(core_intrinsics)]
// code styles
#![allow(dead_code, unused_imports, unused_variables)]
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(noop_method_call)]
#![warn(unused_qualifications)]
#![warn(non_ascii_idents)]
#![deny(unused_must_use)]
#![deny(pointer_structural_match)]

use std::ffi::{OsStr, OsString};
use std::io::Read;
use std::iter::Iterator;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Relaxed, SeqCst};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::de::Unexpected::Seq;

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
use crate::meta::StepArguments;
use crate::mind::gods::bio::GodOfBio;
use crate::mob::bio::bio::Bio;
use crate::mob::bio::species::Species;
use crate::observe::plate::PlateView;
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

    mob_debugger(meta, |_| true, |_| false);
    //benchmark(meta);
}

fn mob_debugger<F: Fn(&MobRef<dyn Mob>) -> bool + Send + Sync, G: Fn(MobRef<dyn Mob>) -> bool>(
    mut meta: MetaCosmos,
    selector: F,
    deselector: G,
) -> ! {
    loop {
        let mob = {
            let found = AtomicBool::new(false);
            let mut val = Mutex::new(None);
            while !found.load(SeqCst) {
                meta.step_x(StepArguments {
                    ground_message_recorder: |_, _| {},
                    mob_message_recorder: |m, _| {
                        if !found.load(Relaxed) && selector(m) && !found.swap(true, SeqCst) {
                            *val.lock().unwrap() = Some(m.downgrade());
                        }
                    },
                    ground_order_recorder: |_, _| {},
                    mob_order_recorder: |_, _| {},
                });
            }
            val.get_mut().unwrap().clone().unwrap()
        };

        while {
            meta.cosmos.pk(|cosmos, pk| {
                ReadGuard::with(pk, |guard| {
                    if let Some(mob) = guard.wrap_weak(&mob) {
                        let at = mob.at();
                        if deselector(mob) {
                            false
                        } else {
                            println!(
                                "{}",
                                PlateView::new(
                                    cosmos,
                                    at.map(|x| Interval::new(x.from - 10, x.to + 10)),
                                    guard
                                )
                            );
                            true
                        }
                    } else {
                        false
                    }
                })
            })
        } {
            meta.step();
        }
    }
}

fn benchmark(mut meta: MetaCosmos) -> ! {
    let mut start = SystemTime::now();
    let mut last_tick = 0;
    loop {
        meta.step();

        let duration = SystemTime::now().duration_since(start).unwrap();
        if duration >= Duration::from_secs(5) {
            println!(
                "ticks {}, cost {}ms",
                meta.cosmos.angelos.properties.tick - last_tick,
                duration.as_millis()
            );
            println!("benchmarks\n{}", meta.benchmark);
            meta.benchmark.clear();
            start = SystemTime::now();
            last_tick = meta.cosmos.angelos.properties.tick;
        }
    }
}
