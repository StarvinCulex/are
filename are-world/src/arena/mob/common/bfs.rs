use std::collections::{HashSet, VecDeque};

use crate::arena::defs::Idx;
use crate::meta::defs::Crd;
use crate::Coord;

pub enum Result<E> {
    Blocked,
    Pass,
    Finish(E),
}

// 断言：地图很大，搜索时产生的坐标不会出现不相等但在棋盘大小上取模后相等的情况
// 按切比雪夫距离展开
pub fn breadth_first_search<R, F: FnMut(Crd) -> Result<R>>(
    start_at: Box<dyn Iterator<Item = Crd>>,
    max_depth: u16,
    mut f: F,
) -> Option<R> {
    const EXPAND: [Crd; 8] = [
        Coord(-1, -1),
        Coord(0, -1),
        Coord(1, -1),
        Coord(1, 0),
        Coord(1, 1),
        Coord(0, 1),
        Coord(-1, 1),
        Coord(-1, 0),
    ];

    let mut visited: HashSet<Crd> = start_at.collect();
    let mut queue: VecDeque<(Crd, u16)> = visited.iter().map(|&x| (x, 0)).collect();
    while let Some((p, depth)) = queue.pop_front() {
        match f(p) {
            Result::Blocked => continue,
            Result::Finish(r) => return Some(r),
            Result::Pass => {
                if max_depth <= depth {
                    break;
                }
                for offset in EXPAND {
                    let child = p + offset;
                    if visited.insert(child) {
                        queue.push_back((child, depth + 1));
                    }
                }
            }
        }
    }

    None
}

#[inline]
pub fn chebyshev_carpet_bomb_search<R, F: FnMut(Crd) -> Option<R>>(
    start_at: Crd,
    max_depth: u16,
    mut f: F,
) -> Option<R> {
    if let Some(a) = f(start_at) {
        return Some(a);
    }

    let max_depth: Idx = max_depth.try_into().unwrap();
    for range in 1..max_depth + 1 {
        if let Some(a) = ((-range..range).map(|x| Coord(x, -range)))
            .chain((-range..range).map(|x| Coord(range, x)))
            .chain((-range..range).map(|x| Coord(-x, range)))
            .chain((-range..range).map(|x| Coord(-range, -x)))
            .map(|x| f(x + start_at))
            .find(Option::is_some)
        {
            return a;
        }
    }
    None
}

#[inline]
pub fn manhattan_carpet_bomb_search<R, F: FnMut(Crd) -> Option<R>>(
    start_at: Crd,
    max_depth: u16,
    mut f: F,
) -> Option<R> {
    if let Some(a) = f(start_at) {
        return Some(a);
    }

    let max_depth: Idx = max_depth.try_into().unwrap();
    for range in 1..max_depth + 1 {
        // top
        if let Some(a) = ((0..range).map(|x| Coord(x, x - range)))
            .chain((0..range).map(|x| Coord(range - x, x)))
            .chain((0..range).map(|x| Coord(-x, range - x)))
            .chain((-range..range).map(|x| Coord(x - range, -x)))
            .map(|x| f(x + start_at))
            .find(Option::is_some)
        {
            return a;
        }
    }
    None
}
