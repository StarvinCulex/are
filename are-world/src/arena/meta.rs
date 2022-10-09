use std::intrinsics::{likely, unlikely};
use std::sync::Arc;
use std::time::Instant;

use crate::arena::defs::Crd;
use crate::stats::bm2::Benchmark;
use crate::{Logger, Mob, MobBlock};

pub use super::*;

pub struct MetaCosmos {
    mind_list: Vec<Box<dyn mind::Mind>>,
    pub cosmos: Cosmos,
}

pub struct StepArguments<
    GMR: Fn(Crd, &[gnd::Msg]) + Send + Sync,
    MMR: Fn(&MobRef<dyn Mob>, &[mob::Msg]) + Send + Sync,
    GOR: Fn(Crd, &[gnd::Order]) + Send + Sync,
    MOR: Fn(&mut MobRefMut<dyn Mob>, &[mob::Order]) + Send + Sync,
> {
    pub ground_message_recorder: GMR,
    pub mob_message_recorder: MMR,
    pub ground_order_recorder: GOR,
    pub mob_order_recorder: MOR,
}

impl MetaCosmos {
    pub fn new(conf: Arc<conf::Conf>) -> MetaCosmos {
        MetaCosmos {
            mind_list: Vec::new(),
            cosmos: Cosmos::new(conf),
        }
    }

    pub fn step(&mut self) {
        self.step_x(StepArguments {
            ground_message_recorder: |_, _| {},
            mob_message_recorder: |_, _| {},
            ground_order_recorder: |_, _| {},
            mob_order_recorder: |_, _| {},
        })
    }

    pub fn step_x<
        GMR: Fn(Crd, &[gnd::Msg]) + Send + Sync,
        MMR: Fn(&MobRef<dyn Mob>, &[mob::Msg]) + Send + Sync,
        GOR: Fn(Crd, &[gnd::Order]) + Send + Sync,
        MOR: Fn(&mut MobRefMut<dyn Mob>, &[mob::Order]) + Send + Sync,
    >(
        &mut self,
        args: StepArguments<GMR, MMR, GOR, MOR>,
    ) {
        macro_rules! bench {
            () => {
                &mut self.cosmos.angelos.stats.get_mut().unwrap().benchmark
            };
        }

        crate::benchmark_time!(
            ["mind move tick", bench!()]
            self.mind_move_tick();
        );

        crate::benchmark_time!(
            ["mind set tick", bench!()]
            self.mind_set_cosmos();
        );

        crate::benchmark_time!(
            ["mind flush tick", bench!()]
            self.mind_flush_queue();
        );

        crate::benchmark_time!(
            ["message tick", bench!()]
            self.cosmos
                .message_tick(args.ground_message_recorder, args.mob_message_recorder);
        );

        crate::benchmark_time!(
            ["order tick", bench!()]
            self.cosmos
                .order_tick(args.ground_order_recorder, args.mob_order_recorder);
        );

        crate::benchmark_time!(
            ["mind view tick", bench!()]
            self.mind_view_tick();
        );

        self.cosmos.add_tick();

        if unlikely(self.cosmos.angelos.conf.runtime.period != 0) {
            std::thread::sleep(std::time::Duration::from_millis(
                self.cosmos.angelos.conf.runtime.period,
            ));
        }
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
