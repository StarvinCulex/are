use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::time::Instant;

pub struct Benchmark {
    records: HashMap<String, u128>,
    last_at: Instant,
}

impl Benchmark {
    pub fn new() -> Self {
        Benchmark {
            records: HashMap::new(),
            last_at: Instant::now(),
        }
    }

    pub fn start(&mut self) {
        self.last_at = Instant::now();
    }

    pub fn clear(&mut self) {
        self.records.clear();
        self.start();
    }

    pub fn end<S: Into<String>>(&mut self, name: S) {
        let t = Instant::now();
        let name = name.into();
        let v = t.duration_since(self.last_at).as_nanos();
        match self.records.entry(name) {
            Entry::Occupied(mut x) => {
                *x.get_mut() = x.get().saturating_add(v);
            }
            Entry::Vacant(x) => {
                x.insert(v);
            }
        };
        self.last_at = Instant::now();
    }
}

impl Display for Benchmark {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (s, v) in self.records.iter() {
            writeln!(f, "{}: {}ns", s, v)?;
        }
        Ok(())
    }
}
