//

pub struct MajorAngelos {
    pub properties: Properties,
    pub plate_size: Coord<usize>,
    pkey: PKey,
    pub species_pool: mob::bio::species::SpeciesPool,

    async_data: Mutex<MajorAngelosAsyncData>,
    mind_waiting_queue: Mutex<VecDeque<Box<dyn mind::Mind>>>,
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

    gnd_messages_buf: Vec<(Tick, Crd, gnd::Msg)>,
    gnd_orders_buf: Vec<(Tick, Crd, gnd::Order)>,
    mob_pos_messages_buf: Vec<(Tick, Crd, mob::Msg)>,
    mob_pos_orders_buf: Vec<(Tick, Crd, mob::Order)>,
    mob_messages_buf: Vec<(Tick, Weak<MobBlock>, mob::Msg)>,
    mob_orders_buf: Vec<(Tick, Weak<MobBlock>, mob::Order)>,
}

impl MajorAngelos {
    pub fn make_worker(&self) -> Angelos {
        Angelos {
            major: self,
            gnd_messages_buf: vec![],
            gnd_orders_buf: vec![],
            mob_pos_messages_buf: vec![],
            mob_pos_orders_buf: vec![],
            mob_messages_buf: vec![],
            mob_orders_buf: vec![],
        }
    }
}

impl<'m> Drop for Angelos<'m> {
    fn drop(&mut self) {
        let mut guard = self.major.async_data.lock().unwrap();
        guard.gnd_orders.append(std::mem::take(&mut self.gnd_orders_buf));
        guard.gnd_messages.append(std::mem::take(&mut self.gnd_messages_buf));
        guard.mob_orders.append(std::mem::take(&mut self.mob_orders_buf));
        guard.mob_messages.append(std::mem::take(&mut self.mob_messages_buf));
        guard.mob_pos_orders.append(std::mem::take(&mut self.mob_pos_orders_buf));
        guard.mob_pos_messages.append(std::mem::take(&mut self.mob_pos_messages_buf));
    }
}

pub trait Teller<Index, Letter> {
    fn tell(&mut self, index: Index, letter: Letter, delay: Tick);
}

pub trait Orderer<Index, Letter> {
    fn order(&mut self, index: Index, letter: Letter, delay: Tick);
}

// Angelos
impl Teller<Crd, gnd::Msg> for Angelos<'_> {
    #[inline]
    fn tell(&mut self, mut k: Crd, v: gnd::Msg, delay: Tick) {
        k = Matrix::<(), 1, 1>::normalize_pos(self.major.plate_size.try_into().unwrap(), k.into())
            .try_into()
            .unwrap();
        self.gnd_messages_buf.push((delay, k, v))
    }
}

impl Orderer<Crd, gnd::Order> for Angelos<'_> {
    #[inline]
    fn order(&mut self, mut k: Crd, v: gnd::Order, delay: Tick) {
        k = Matrix::<(), 1, 1>::normalize_pos(self.major.plate_size.try_into().unwrap(), k.into())
            .try_into()
            .unwrap();
        self.gnd_orders_buf.push((delay, k, v))
    }
}

impl Teller<Crd, mob::Msg> for Angelos<'_> {
    #[inline]
    fn tell(&mut self, mut k: Crd, v: mob::Msg, delay: Tick) {
        k = Matrix::<(), 1, 1>::normalize_pos(self.major.plate_size.try_into().unwrap(), k.into())
            .try_into()
            .unwrap();
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
        k = Matrix::<(), 1, 1>::normalize_pos(self.major.plate_size.try_into().unwrap(), k.into())
            .try_into()
            .unwrap();
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
    pub(crate) fn flush_minds(&mut self) -> VecDeque<Box<dyn mind::Mind>> {
        std::mem::take(self.mind_waiting_queue.get_mut().unwrap())
    }
    pub fn join(&mut self, mind: Box<dyn mind::Mind>) {
        self.mind_waiting_queue.get_mut().unwrap().push_back(mind)
    }
    pub fn join_lock(&self, mind: Box<dyn mind::Mind>) {
        self.mind_waiting_queue.lock().unwrap().push_back(mind)
    }
}
