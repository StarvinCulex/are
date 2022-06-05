use std::intrinsics::likely;
use std::sync::Arc;
use std::time::Instant;

use crate::arena::conf::GameConf;
use crate::stats::benchmark::Benchmark;

pub use super::*;

pub struct MetaCosmos {
    mind_list: Vec<Box<dyn mind::Mind>>,
    pub cosmos: Cosmos,
    pub benchmark: Benchmark,
}

impl MetaCosmos {
    pub fn new(conf: Arc<conf::Conf>) -> MetaCosmos {
        MetaCosmos {
            mind_list: Vec::new(),
            cosmos: Cosmos::new(conf),
            benchmark: Benchmark::new(),
        }
    }

    pub fn step(&mut self) {
        self.benchmark.clear();
        self.mind_move_tick();
        self.benchmark.end("mind move tick");
        self.mind_set_cosmos();
        self.benchmark.end("mind set tick");
        self.mind_flush_queue();
        self.benchmark.end("mind flush tick");
        self.cosmos.message_tick();
        self.benchmark.end("message tick");
        self.cosmos.order_tick();
        self.benchmark.end("order tick");
        self.mind_view_tick();
        self.benchmark.end("mind view tick");
        self.cosmos.add_tick();
        self.benchmark.end("add tick");
    }
}

// private

impl MetaCosmos {
    #[inline]
    fn mind_flush_queue(&mut self) {
        let mind_queue = self.cosmos.flush_minds();
        for mind in mind_queue {
            self.mind_list.push(mind);
        }
    }

    #[inline]
    fn mind_view_tick(&mut self) {
        let cap = self.mind_list.len();
        let old_mind_list = std::mem::replace(&mut self.mind_list, Vec::with_capacity(cap));
        self.cosmos.pk(|cosmos, pkey| {
            for mut mind in old_mind_list {
                if likely(mind.observe(cosmos, pkey).is_ok()) {
                    self.mind_list.push(mind);
                }
            }
        })
    }

    #[inline]
    fn mind_move_tick(&mut self) {
        let cap = self.mind_list.len();
        let old_mind_list = std::mem::replace(&mut self.mind_list, Vec::with_capacity(cap));
        self.cosmos.pk(|cosmos, pkey| {
            for mut mind in old_mind_list {
                if likely(mind.make_move(cosmos, pkey).is_ok()) {
                    self.mind_list.push(mind);
                }
            }
        })
    }

    #[inline]
    fn mind_set_cosmos(&mut self) {
        let cap = self.mind_list.len();
        let old_mind_list = std::mem::replace(&mut self.mind_list, Vec::with_capacity(cap));
        for mut mind in old_mind_list {
            if likely(mind.set_cosmos(&mut self.cosmos).is_ok()) {
                self.mind_list.push(mind);
            }
        }
    }
}
