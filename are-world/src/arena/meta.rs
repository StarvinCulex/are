use crate::arena::conf::StaticConf;

pub use super::*;

pub struct MetaCosmos {
    mind_list: Vec<Box<dyn mind::Mind>>,
    pub cosmos: Cosmos,
}

impl MetaCosmos {
    pub fn new(static_conf: StaticConf, runtime_conf: RuntimeConf) -> MetaCosmos {
        MetaCosmos {
            mind_list: Vec::new(),
            cosmos: Cosmos::new(static_conf, runtime_conf),
        }
    }

    pub fn step(&mut self) {
        self.mind_move_tick();
        self.mind_set_cosmos();
        self.mind_flush_queue();
        self.cosmos.hear_tick();
        self.cosmos.watch_act_tick();
        self.mind_view_tick();
        self.cosmos.angelos.properties.tick += 1;
    }
}

// private

impl MetaCosmos {
    #[inline]
    fn mind_flush_queue(&mut self) {
        let mind_queue = self.cosmos.angelos.flush_minds();
        for mind in mind_queue {
            self.mind_list.push(mind);
        }
    }

    #[inline]
    fn mind_view_tick(&mut self) {
        let cap = self.mind_list.len();
        let old_mind_list = std::mem::replace(&mut self.mind_list, Vec::with_capacity(cap));
        for mut mind in old_mind_list {
            if mind.observe(&self.cosmos).is_ok() {
                self.mind_list.push(mind);
            }
        }
    }

    #[inline]
    fn mind_move_tick(&mut self) {
        let cap = self.mind_list.len();
        let old_mind_list = std::mem::replace(&mut self.mind_list, Vec::with_capacity(cap));
        for mut mind in old_mind_list {
            if mind.make_move(&self.cosmos).is_ok() {
                self.mind_list.push(mind);
            }
        }
    }

    #[inline]
    fn mind_set_cosmos(&mut self) {
        let cap = self.mind_list.len();
        let old_mind_list = std::mem::replace(&mut self.mind_list, Vec::with_capacity(cap));
        for mut mind in old_mind_list {
            if mind.set_cosmos(&mut self.cosmos).is_ok() {
                self.mind_list.push(mind);
            }
        }
    }
}
