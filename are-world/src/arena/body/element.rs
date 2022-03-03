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
    // TODO: remove self, &mut Cosmos
    pub fn hear(&self, cosmos: &Cosmos, at: Coord<isize>, message: &Message) {
        match message {
            Message::Fire => {
                if self.light() {
                    cosmos
                        .angelos
                        .wake(at, Self::this_fire_tick(&cosmos.angelos))
                }
            }
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
    pub fn burn(cosmos: &mut Cosmos, at: Coord<isize>) {
        if cosmos.plate[at].body.element.data.fetch_or(IS_FIRE, SeqCst) & IS_FIRE == 0 {
            cosmos.angelos.tell_area((at - Coord(1, 1)) | (at + Coord(1, 1)), Message::Fire);
        }
    }

    pub fn is_burning(&self) -> bool {
        self.data.load(SeqCst) & IS_FIRE != 0
    }

    // TODO: &mut self
    #[inline]
    pub fn light(&self) -> bool {
        self.data.fetch_add(1, SeqCst) & FIRE_COUNT == 0
    }

    #[inline]
    fn fire_tick(&mut self, pos: Coord<isize>, angelos: &Angelos) {
        let burning = self.is_burning();
        // 邻居个数（不含自己）
        let neighbors = (*self.data.get_mut() & FIRE_COUNT) - (if burning { 1 } else { 0 });
        if burning {
            println!("{} burning, neighbors={}", pos, neighbors);
            angelos.tell_area((pos - Coord(1, 1)) | (pos + Coord(1, 1)), Message::Fire);
        }
        // 3 个邻居 -> 复活
        // 2 个邻居 -> 稳定
        // <2 个邻居 -> 孤单死亡
        // >3 个邻居 -> 拥挤死亡
        match neighbors {
            3 => {
                // live
                if !burning {
                    angelos.wake(pos, Self::next_fire_tick(angelos));
                }
                *self.data.get_mut() = IS_FIRE
            }
            2 => {
                // keep
                *self.data.get_mut() &= IS_FIRE
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
