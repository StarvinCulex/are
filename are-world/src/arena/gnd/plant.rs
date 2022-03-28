use std::sync::atomic::{AtomicU8, Ordering};

use crate::arena::defs::Crd;
use crate::arena::{Angelos, Cosmos};
use crate::Coord;

pub struct Plant {
    pub kind: Kind,
    pub age: AtomicU8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    None,
    Grass,
}
//
// impl Plant {
//     #[inline]
//     pub fn age(&self, cosmos: &Cosmos, pos: Crd) {
//         let last_age = self.age.fetch_add(1, Ordering::Relaxed);
//         if last_age >= self.kind.max_age() {
//             cosmos.angelos.wake(pos, 0);
//         }
//     }
//
//     #[inline]
//     fn breed(&mut self, at: Crd, angelos: &Angelos) {
//         // match self.kind.clone() {
//         //     Kind::None => (),
//         //     Kind::Grass => angelos.tell_area(
//         //         (at - Coord(1, 1)) | (at + Coord(1, 1)),
//         //         message::Message::PlantSowing(self.kind),
//         //     ),
//         // }
//         // 不能这么做。Kind如此设定就必须在act tick进行更改操作。
//     }
//
//     #[inline]
//     fn sow(&mut self, at: Crd, angelos: &Angelos) {}
// }
//
// impl Kind {
//     #[inline]
//     fn max_age(&self) -> u8 {
//         match self {
//             Kind::None => 255,
//             Kind::Grass => 16,
//         }
//     }
// }
