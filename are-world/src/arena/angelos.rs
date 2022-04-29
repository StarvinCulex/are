//

pub struct MajorAngelos {
    pub properties: Properties,
    pub plate_size: Coord<usize>,
    pkey: PKey,
    pub species_pool: mob::bio::species::SpeciesPool,

    pub async_data: Mutex<MajorAngelosAsyncData>,
}

pub struct MajorAngelosAsyncData {
    gnd_messages: MPipe<Crd, gnd::Msg>,
    gnd_orders: MPipe<Crd, gnd::Order>,

    mob_pos_messages: MPipe<Crd, mob::Msg>,
    mob_pos_orders: MPipe<Crd, mob::Order>,
    mob_messages: MPipe<Weak<MobBlock>, mob::Msg>,
    mob_orders: MPipe<Weak<MobBlock>, mob::Order>,

    mind_waiting_queue: VecDeque<Box<dyn mind::Mind>>,
}

pub struct Angelos<'m> {
    pub properties: &'m Properties,
    pub plate_size: &'m Coord<usize>,
    pub species_pool: &'m mob::bio::species::SpeciesPool,
    pkey: PKey,

    gnd_messages_buf: Vec<(Tick, Crd, gnd::Msg)>,
    gnd_orders_buf: Vec<(Tick, Crd, gnd::Order)>,
    mob_pos_messages_buf: Vec<(Tick, Crd, mob::Msg)>,
    mob_pos_orders_buf: Vec<(Tick, Crd, mob::Order)>,
    mob_messages_buf: Vec<(Tick, Weak<MobBlock>, mob::Msg)>,
    mob_orders_buf: Vec<(Tick, Weak<MobBlock>, mob::Order)>,

    mind_waiting_buf: Vec<Box<dyn mind::Mind>>,
}

impl MajorAngelos {
    fn bunshin(&self) -> Angelos {
        Angelos {
            properties: &self.properties,
            plate_size: &self.plate_size,
            species_pool: &self.species_pool,
            pkey: PKey::new(),
            gnd_messages_buf: vec![],
            gnd_orders_buf: vec![],
            mob_pos_messages_buf: vec![],
            mob_pos_orders_buf: vec![],
            mob_messages_buf: vec![],
            mob_orders_buf: vec![],
            mind_waiting_buf: vec![],
        }
    }

    fn consume<'a>(&'a self, angelos: Angelos<'a>) {
        let mut guard = self.async_data.lock().unwrap();
        guard
            .mind_waiting_queue
            .append(&mut angelos.mind_waiting_buf.into());
        guard.gnd_orders.append(angelos.gnd_orders_buf);
        guard.gnd_messages.append(angelos.gnd_messages_buf);
        guard.mob_orders.append(angelos.mob_orders_buf);
        guard.mob_messages.append(angelos.mob_messages_buf);
        guard.mob_pos_orders.append(angelos.mob_pos_orders_buf);
        guard.mob_pos_messages.append(angelos.mob_pos_messages_buf);
    }
}

impl Angelos<'_> {
    pub fn join(&mut self, mind: Box<dyn mind::Mind>) {
        self.mind_waiting_buf.push(mind)
    }
}

pub trait Teller<Index, Letter> {
    fn tell(&mut self, index: Index, letter: Letter, delay: Tick);
}

pub trait Orderer<Index, Letter> {
    fn order(&mut self, index: Index, letter: Letter, delay: Tick);
}

impl Teller<Crd, gnd::Msg> for Angelos<'_> {
    #[inline]
    fn tell(&mut self, mut k: Crd, v: gnd::Msg, delay: Tick) {
        k = Matrix::<(), 1, 1>::normalize_pos((*self.plate_size).try_into().unwrap(), k.into())
            .try_into()
            .unwrap();
        self.gnd_messages_buf.push((delay, k, v))
    }
}

impl Orderer<Crd, gnd::Order> for Angelos<'_> {
    #[inline]
    fn order(&mut self, mut k: Crd, v: gnd::Order, delay: Tick) {
        k = Matrix::<(), 1, 1>::normalize_pos((*self.plate_size).try_into().unwrap(), k.into())
            .try_into()
            .unwrap();
        self.gnd_orders_buf.push((delay, k, v))
    }
}

impl Teller<Crd, mob::Msg> for Angelos<'_> {
    #[inline]
    fn tell(&mut self, mut k: Crd, v: mob::Msg, delay: Tick) {
        k = Matrix::<(), 1, 1>::normalize_pos((*self.plate_size).try_into().unwrap(), k.into())
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
        k = Matrix::<(), 1, 1>::normalize_pos((*self.plate_size).try_into().unwrap(), k.into())
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
        std::mem::take(&mut self.async_data.get_mut().unwrap().mind_waiting_queue)
    }
}
