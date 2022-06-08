//! by *StarvinCulex @2021/11/14*
use super::{Coord, Interval};

pub trait Accessor<const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> {
    fn next(&mut self) -> Option<(Coord<isize>, usize)>;

    /// 同 ExactSizeIterator::len
    fn len(&self) -> usize;

    fn super_area(&self) -> Coord<Interval<isize>>;

    /// 注意先 normalize 一下 pos
    fn contains(&self, pos: Coord<isize>) -> bool;

    fn r#type(&self) -> &'static str;
}
