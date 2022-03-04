use std::collections::VecDeque;

use crate::arena::conf::StaticConf;

pub use super::*;

type Hasher = std::collections::hash_map::RandomState;

pub struct Cosmos {
    pub plate: Matrix<Block, 1, 1>,
    pub angelos: Angelos,
}

#[derive(Default)]
pub struct Block {
    pub body: body::Body,
}

pub struct Angelos {
    pub properties: Properties,
    wake_positions: Mutex<
        std::collections::HashMap<u64, std::collections::HashSet<Coord<isize>, Hasher>, Hasher>,
    >,
    messages: Mutex<Vec<(Coord<Interval<isize>>, Message)>>,

    plate_size: Coord<isize>,

    mind_waiting_queue: Mutex<VecDeque<Box<dyn mind::Mind>>>,
}

pub struct Properties {
    pub tick: u64,
    pub runtime_conf: RuntimeConf,
}

impl Angelos {
    pub fn wake(&self, pos: Coord<isize>, next: u64) {
        let normalized_pos = pos.reduce(self.plate_size, isize::rem_euclid);
        let at = self.properties.tick + next;

        let mut wake_positions = self.wake_positions.lock().unwrap();
        if let Some(set) = wake_positions.get_mut(&at) {
            set.insert(normalized_pos);
        } else {
            wake_positions.insert(at, std::collections::HashSet::from([normalized_pos]));
        }
    }

    pub fn tell_area(&self, area: Coord<Interval<isize>>, message: Message) {
        self.messages.lock().unwrap().push((area, message))
    }

    #[inline]
    #[allow(clippy::eq_op)]
    pub fn tell(&self, pos: Coord<isize>, message: Message) {
        self.tell_area(pos | pos, message)
    }

    pub fn join(&self, mind: Box<dyn mind::Mind>) {
        self.mind_waiting_queue.lock().unwrap().push_back(mind)
    }
}

impl Cosmos {
    pub fn new(static_conf: StaticConf, runtime_conf: RuntimeConf) -> Self {
        Cosmos {
            plate: Matrix::new(&static_conf.plate_size),
            angelos: Angelos {
                properties: Properties {
                    tick: 0,
                    runtime_conf,
                },
                wake_positions: Mutex::default(),
                messages: Mutex::default(),
                plate_size: static_conf.plate_size.try_into().unwrap(),
                mind_waiting_queue: Mutex::default(),
            },
        }
    }
}

impl Cosmos {
    #[inline]
    pub(crate) fn hear_tick(&mut self) {
        let messages: Vec<(Coord<Interval<isize>>, Message)> =
            std::mem::take(self.angelos.messages.lock().unwrap().as_mut());
        for (area, message) in messages {
            let normalized_area = self.plate.normalize_area(area);
            for (pos, block) in self.plate.area(normalized_area) {
                block.body.hear(self, pos, &message)
            }
        }
    }

    #[inline]
    pub(crate) fn watch_act_tick(&mut self) {
        if let Some(wake_positions) = self
            .angelos
            .wake_positions
            .get_mut()
            .unwrap()
            .remove(&self.angelos.properties.tick)
        {
            // watch tick
            for &pos in wake_positions.iter() {
                self.plate[pos].body.watch(self, pos);
            }

            // act tick
            for &pos in wake_positions.iter() {
                self.plate[pos].body.act(pos, &self.angelos);
            }
        }
    }

    // TODO: move this away
    pub fn burn(&mut self, at: Coord<isize>) {
        self.plate[at].body.element.ignite(self, at);
    }
}

impl Angelos {
    pub(crate) fn flush_minds(&mut self) -> VecDeque<Box<dyn mind::Mind>> {
        std::mem::replace(self.mind_waiting_queue.get_mut().unwrap(), VecDeque::new())
    }
}
