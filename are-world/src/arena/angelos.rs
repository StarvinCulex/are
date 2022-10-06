//

use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::singletons::Singletons;
use crate::stat::Stats;
use crate::stats::bm2::Benchmark;

pub struct MajorAngelos {
    pub properties: Properties,
    pub conf: Arc<conf::Conf>,
    pub plate_size: Coord<Idx>,
    pub singletons: Singletons,
    pub stats: Mutex<Stats>,
    pkey: PKey,

    async_data: Mutex<MajorAngelosAsyncData>,
    mind_waiting_queue: Mutex<VecDeque<Box<dyn Mind>>>,
}

pub struct MajorAngelosAsyncData {
    gnd_messages: MPipe<Crd, gnd::Msg>,
    gnd_orders: MPipe<Crd, gnd::Order>,

    mob_pos_messages: MPipe<Crd, mob::Msg>,
    mob_pos_orders: MPipe<Crd, mob::Order>,
    mob_messages: MPipe<Weak<MobBlock>, mob::Msg>,
    mob_orders: MPipe<Weak<MobBlock>, mob::Order>,
}

pub struct Angelos<'m> {
    pub major: &'m MajorAngelos,
    pub random: StdRng,
    stats: Stats,

    gnd_messages_buf: Vec<(Tick, Crd, gnd::Msg)>,
    gnd_orders_buf: Vec<(Tick, Crd, gnd::Order)>,
    mob_pos_messages_buf: Vec<(Tick, Crd, mob::Msg)>,
    mob_pos_orders_buf: Vec<(Tick, Crd, mob::Order)>,
    mob_messages_buf: Vec<(Tick, Weak<MobBlock>, mob::Msg)>,
    mob_orders_buf: Vec<(Tick, Weak<MobBlock>, mob::Order)>,
}

impl MajorAngelos {
    #[inline]
    pub fn make_worker(&self) -> Angelos {
        Angelos {
            major: self,
            random: StdRng::from_entropy(),
            stats: Stats::new(),
            gnd_messages_buf: vec![],
            gnd_orders_buf: vec![],
            mob_pos_messages_buf: vec![],
            mob_pos_orders_buf: vec![],
            mob_messages_buf: vec![],
            mob_orders_buf: vec![],
        }
    }

    #[inline]
    pub fn normalize_pos(&self, pos: Crd) -> Crd {
        Matrix::<(), CHUNK_WIDTH, CHUNK_HEIGHT>::normalize_pos(self.plate_size.into(), pos.into())
            .try_into()
            .unwrap()
    }

    #[inline]
    pub fn normalize_area(&self, area: CrdI) -> CrdI {
        Matrix::<(), CHUNK_WIDTH, CHUNK_HEIGHT>::normalize_area_with(
            self.plate_size.into(),
            area.into(),
        )
        .try_into()
        .unwrap()
    }
}

impl<'m> Drop for Angelos<'m> {
    #[inline]
    fn drop(&mut self) {
        *self.major.stats.lock().unwrap() += std::mem::take(&mut self.stats);

        let mut guard = self.major.async_data.lock().unwrap();
        guard
            .gnd_orders
            .append(std::mem::take(&mut self.gnd_orders_buf));
        guard
            .gnd_messages
            .append(std::mem::take(&mut self.gnd_messages_buf));
        guard
            .mob_orders
            .append(std::mem::take(&mut self.mob_orders_buf));
        guard
            .mob_messages
            .append(std::mem::take(&mut self.mob_messages_buf));
        guard
            .mob_pos_orders
            .append(std::mem::take(&mut self.mob_pos_orders_buf));
        guard
            .mob_pos_messages
            .append(std::mem::take(&mut self.mob_pos_messages_buf));
    }
}

pub trait AngelosStat {
    fn stats(&mut self) -> &mut Stats;
}

pub trait Teller<Index, Letter> {
    fn tell(&mut self, index: Index, letter: Letter, delay: Tick);
}

pub trait Orderer<Index, Letter> {
    fn order(&mut self, index: Index, letter: Letter, delay: Tick);
}

// Angelos
impl AngelosStat for MajorAngelos {
    fn stats(&mut self) -> &mut Stats {
        self.stats.get_mut().unwrap()
    }
}

impl<'m> AngelosStat for Angelos<'m> {
    #[inline]
    fn stats(&mut self) -> &mut Stats {
        &mut self.stats
    }
}

impl Teller<Crd, gnd::Msg> for Angelos<'_> {
    #[inline]
    fn tell(&mut self, mut k: Crd, v: gnd::Msg, delay: Tick) {
        k = self.major.normalize_pos(k);
        self.gnd_messages_buf.push((delay, k, v))
    }
}

impl Orderer<Crd, gnd::Order> for Angelos<'_> {
    #[inline]
    fn order(&mut self, mut k: Crd, v: gnd::Order, delay: Tick) {
        k = self.major.normalize_pos(k);
        self.gnd_orders_buf.push((delay, k, v))
    }
}

impl Teller<Crd, mob::Msg> for Angelos<'_> {
    #[inline]
    fn tell(&mut self, mut k: Crd, v: mob::Msg, delay: Tick) {
        k = self.major.normalize_pos(k);
        self.mob_pos_messages_buf.push((delay, k, v))
    }
}

impl Teller<Weak<MobBlock>, mob::Msg> for Angelos<'_> {
    #[inline]
    fn tell(&mut self, k: Weak<MobBlock>, v: mob::Msg, delay: Tick) {
        self.mob_messages_buf.push((delay, k, v))
    }
}

impl Orderer<Crd, mob::Order> for Angelos<'_> {
    #[inline]
    fn order(&mut self, mut k: Crd, v: mob::Order, delay: Tick) {
        k = self.major.normalize_pos(k);
        self.mob_pos_orders_buf.push((delay, k, v))
    }
}

impl Orderer<Weak<MobBlock>, mob::Order> for Angelos<'_> {
    #[inline]
    fn order(&mut self, k: Weak<MobBlock>, v: mob::Order, delay: Tick) {
        self.mob_orders_buf.push((delay, k, v))
    }
}

impl MajorAngelos {
    pub(crate) fn flush_minds(&mut self) -> VecDeque<Box<dyn Mind>> {
        std::mem::take(self.mind_waiting_queue.get_mut().unwrap())
    }
    pub fn join(&mut self, mind: Box<dyn Mind>) {
        self.mind_waiting_queue.get_mut().unwrap().push_back(mind)
    }
    pub fn join_lock(&self, mind: Box<dyn Mind>) {
        self.mind_waiting_queue.lock().unwrap().push_back(mind)
    }
}
