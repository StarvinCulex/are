use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use std::intrinsics::saturating_add;
use std::ops::AddAssign;
use std::time::Instant;

use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub enum Item {
    Count(u64),
    CPUCost {
        count: u64,
        nanos: u128,
        #[serde(skip)]
        last_start: Instant,
    },
}

#[derive(Clone, Default, Serialize)]
pub struct Benchmark {
    #[cfg(feature = "benchmark")]
    pub data: BTreeMap<String, Item>,
}

impl Benchmark {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(feature = "benchmark")]
impl Benchmark {
    pub fn count<S: ToString>(&mut self, key: S, add: u64) -> Result<u64, ()> {
        match self.data.entry(key.to_string()).or_insert(Item::Count(0)) {
            Item::Count(c) => {
                *c = (*c).saturating_add(add);
                Ok(*c)
            }
            _ => Err(()),
        }
    }

    pub fn start_timing<S: ToString>(&mut self, key: S) -> Result<(), ()> {
        match self.data.entry(key.to_string()).or_insert(Item::CPUCost {
            count: 0,
            nanos: 0,
            last_start: Instant::now(),
        }) {
            Item::CPUCost { last_start, .. } => {
                *last_start = Instant::now();
                Ok(())
            }
            _ => Err(()),
        }
    }

    pub fn stop_timing<S: ToString>(&mut self, key: S) -> Result<u128, ()> {
        let t = Instant::now();
        match self.data.entry(key.to_string()).or_insert(Item::CPUCost {
            count: 0,
            nanos: 0,
            last_start: Instant::now(),
        }) {
            Item::CPUCost {
                count,
                nanos,
                last_start,
            } => {
                let v = t.duration_since(*last_start).as_nanos();
                *count += 1;
                *nanos += v;
                *last_start = Instant::now();
                Ok(v)
            }
            _ => Err(()),
        }
    }
}

#[cfg(not(feature = "benchmark"))]
impl Benchmark {
    pub fn count<S: ToString>(&mut self, key: S, add: u64) -> Result<u64, ()> {
        Ok(0)
    }

    pub fn start_timing<S: ToString>(&mut self, key: S) -> Result<(), ()> {
        Ok(())
    }

    pub fn stop_timing<S: ToString>(&mut self, key: S) -> Result<u128, ()> {
        Ok(0)
    }
}

impl Display for Benchmark {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        #[cfg(feature = "benchmark")]
        for (key, value) in self.data.iter() {
            match *value {
                Item::Count(count) => {
                    writeln!(f, "{k}: {v}", k = key, v = count)?;
                }
                Item::CPUCost { count, nanos, .. } => {
                    writeln!(
                        f,
                        "{k}: {total}ms / {count} = {avg}ms",
                        k = key,
                        total = nanos as f64 / 1_000_000.0,
                        count = count,
                        avg = (nanos / count as u128) as f64 / 1_000_000.0,
                    )?;
                }
            }
        }
        Ok(())
    }
}

impl Debug for Benchmark {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl std::ops::Add for Benchmark {
    type Output = Self;
    #[inline]
    fn add(mut self, rhs: Benchmark) -> Self {
        self.add_assign(rhs);
        self
    }
}

impl AddAssign for Benchmark {
    fn add_assign(&mut self, rhs: Self) {
        #[cfg(feature = "benchmark")]
        for (k, rv) in rhs.data {
            match self.data.entry(k.clone()) {
                Entry::Vacant(e) => {
                    e.insert(rv);
                }
                Entry::Occupied(mut e) => match e.get_mut() {
                    Item::Count(lc) => match rv {
                        Item::Count(rc) => *lc = lc.saturating_add(rc),
                        _ => {}
                    },
                    Item::CPUCost {
                        count,
                        nanos,
                        last_start,
                    } => match rv {
                        Item::CPUCost {
                            count: rc,
                            nanos: rn,
                            last_start: rl,
                        } => {
                            *count = count.saturating_add(rc);
                            *nanos = nanos.saturating_add(rn);
                            *last_start = std::cmp::max(*last_start, rl);
                        }
                        _ => {}
                    },
                },
            }
        }
    }
}

#[macro_export]
macro_rules! benchmark_time {
    ([$key: expr, $benchmark: expr] $($tts: tt)*) => {
        #[cfg(feature = "benchmark")]
        $crate::stats::bm2::Benchmark::start_timing($benchmark, $key).unwrap();
        $($tts)*
        #[cfg(feature = "benchmark")]
        $crate::stats::bm2::Benchmark::stop_timing($benchmark, $key).unwrap();
    };
}
