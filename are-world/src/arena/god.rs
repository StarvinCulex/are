use crate::arena::conf::StaticConf;
use crate::arena::*;

pub struct God {
    pub cosmos: Cosmos,
    last_tick: std::time::SystemTime,
}

impl God {
    pub fn new(static_conf: StaticConf, runtime_conf: RuntimeConf) -> God {
        todo!()
    }

    pub fn next_tick(&mut self) {
        let now = std::time::SystemTime::now();
        let last_tick = std::mem::replace(&mut self.last_tick, now);
        if let Ok(spent) = now.duration_since(last_tick) {
            let period = std::time::Duration::from_millis(
                self.cosmos.angelos.properties.runtime_conf.period,
            );
            if period > spent {
                std::thread::sleep(period - spent);
            }
        }
    }
}
