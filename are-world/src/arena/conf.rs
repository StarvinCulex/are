pub struct StaticConf {
    pub plate_size: crate::grids::Coord<usize>,
}

pub struct RuntimeConf {
    pub period: u64,
    pub fire_tick: u64,
    pub thread_count: u64,
}
