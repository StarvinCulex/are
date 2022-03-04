use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::Ordering::SeqCst;

use crate::arena::*;

const FIRE_COUNT: u32 = 0x0F;
const IS_FIRE: u32 = 0x10;

pub struct Element {
    /// 0x0F: 火焰计数信号
    /// 0x10: 是否是火焰
    data: std::sync::atomic::AtomicU32,
}

impl Element {
    pub fn hear(&self, cosmos: &Cosmos, at: Coord<isize>, message: &Message) {
        match message {
            Message::Light => self.light(cosmos, at),
            Message::Ignite => self.ignite(cosmos, at),
        }
    }

    pub fn act(&mut self, pos: Coord<isize>, angelos: &Angelos) {
        self.fire_tick(pos, angelos)
    }

    pub fn get_raw(&self) -> u32 {
        self.data.load(SeqCst)
    }

    pub fn set_raw(&mut self, value: u32) {
        self.data.store(value, SeqCst)
    }
    #[inline]
    pub fn get_name(&self) -> SWord {
        if self.is_burning() {
            "fire".into()
        } else {
            "".into()
        }
    }
}

impl Element {
    pub fn ignite(&self, cosmos: &Cosmos, at: Coord<isize>) {
        if self.data.fetch_or(IS_FIRE, SeqCst) & IS_FIRE == 0 {
            cosmos
                .angelos
                .tell_area((at - Coord(1, 1)) | (at + Coord(1, 1)), Message::Light);
        }
    }

    pub fn is_burning(&self) -> bool {
        self.data.load(SeqCst) & IS_FIRE != 0
    }

    #[inline]
    pub fn light(&self, cosmos: &Cosmos, at: Coord<isize>) {
        let not_lit = self.data.fetch_add(1, SeqCst) & FIRE_COUNT == 0;
        if not_lit {
            cosmos
                .angelos
                .wake(at, Self::next_fire_tick(&cosmos.angelos))
        }
    }

    #[inline]
    fn fire_tick(&mut self, pos: Coord<isize>, angelos: &Angelos) {
        let data = *self.data.get_mut();
        let mut burning = data & IS_FIRE != 0;
        // 邻居个数（不含自己）
        let neighbors = (data & FIRE_COUNT) - (burning as u32);
        // 3 个邻居 -> 复活
        // 2 个邻居 -> 稳定
        // <2 个邻居 -> 孤单死亡
        // >3 个邻居 -> 拥挤死亡
        *self.data.get_mut() = match neighbors {
            3 => {
                // live
                burning = true;
                IS_FIRE
            }
            2 => {
                // keep
                data & IS_FIRE
            }
            _ => {
                // dead
                burning = false;
                0
            }
        };
        if burning {
            angelos.tell_area((pos - Coord(1, 1)) | (pos + Coord(1, 1)), Message::Light);
        }
    }

    #[inline]
    fn next_fire_tick(angelos: &Angelos) -> u64 {
        angelos.properties.runtime_conf.fire_tick
            - angelos.properties.tick % angelos.properties.runtime_conf.fire_tick
    }
}

impl Default for Element {
    fn default() -> Self {
        Element { data: 0.into() }
    }
}
