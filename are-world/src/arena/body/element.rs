use std::sync::atomic::Ordering::Relaxed;

use crate::arena::*;

pub struct Element {
    /// 0x0F: 火焰计数信号
    /// 0x10: 是否是火焰
    data: std::sync::atomic::AtomicU32,
}

impl Element {
    pub fn hear(&self, cosmos: &Cosmos, at: Coord<isize>, message: &Message) {
        match message {
            Message::Fire => {
                if self.light() {
                    cosmos
                        .angelos
                        .wake(at, Self::next_fire_tick(&cosmos.angelos))
                }
            }
        }
    }

    pub fn act(&mut self, pos: Coord<isize>, angelos: &Angelos) {
        self.fire_tick(pos, angelos)
    }

    pub fn get_raw(&self) -> u32 {
        self.data.load(Relaxed)
    }

    pub fn set_raw(&mut self, value: u32) {
        self.data.store(value, Relaxed)
    }
    #[inline]
    pub fn get_name(&self) -> SWord {
        if self.light() {
            "fire".into()
        } else {
            "air".into()
        }
    }
}

impl Element {
    pub fn is_burning(&self) -> bool {
        self.data.load(Relaxed) & 0x10 != 0
    }

    #[inline]
    fn light(&self) -> bool {
        self.data.fetch_add(1, Relaxed) & 0x0F == 1
    }

    #[inline]
    fn fire_tick(&mut self, pos: Coord<isize>, angelos: &Angelos) {
        if self.is_burning() {
            angelos.tell_area((pos - Coord(1, 1)) | (pos + Coord(1, 1)), Message::Fire);
            angelos.wake(pos, Self::next_fire_tick(angelos));
        }
        match *self.data.get_mut() & 0x7F {
            3 => {
                // live
                *self.data.get_mut() = 0x80
            }
            4 => {
                // keep
                *self.data.get_mut() &= 0x80
            }
            _ => {
                // dead
                *self.data.get_mut() = 0
            }
        }
    }

    #[inline]
    fn this_fire_tick(angelos: &Angelos) -> u64 {
        angelos.properties.tick / angelos.properties.runtime_conf.fire_tick
            + angelos.properties.runtime_conf.fire_tick
            - 1
            - angelos.properties.tick
    }
    #[inline]
    fn next_fire_tick(angelos: &Angelos) -> u64 {
        angelos.properties.tick / angelos.properties.runtime_conf.fire_tick
            + angelos.properties.runtime_conf.fire_tick * 2
            - 1
            - angelos.properties.tick
    }
}

impl Default for Element {
    fn default() -> Self {
        Element { data: 0.into() }
    }
}
