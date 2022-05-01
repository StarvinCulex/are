use super::pipe;
use super::MPipe;
use crate::msgpip::pipe::Output;
use crate::{Coord, Matrix};

/// F是转换成区块坐标的函数
pub struct CMPipe<K: std::cmp::Eq + std::hash::Hash, V, F: Fn(&K) -> Coord<isize>> {
    data: Matrix<MPipe<K, V>, 1, 1>,
    f: F,
}

pub struct COutput<K: std::cmp::Eq + std::hash::Hash, V, F: Fn(&K) -> Coord<isize>> {
    f: F,
    data: Matrix<pipe::Output<K, V>, 1, 1>,
}

impl<K: std::cmp::Eq + std::hash::Hash, V, F: Fn(&K) -> Coord<isize>> CMPipe<K, V, F> {
    pub fn new(chunk_count: Coord<usize>, f: F) -> Self {
        CMPipe {
            f,
            data: Matrix::new(chunk_count),
        }
    }

    pub fn push(&mut self, delay: u64, key: K, value: V) {
        let chunk_coord = (self.f)(&key);
        self.data[chunk_coord].push(delay, key, value);
    }

    pub fn append(&mut self, list: Vec<(u64, K, V)>) {
        for (t, k, v) in list {
            self.push(t, k, v);
        }
    }

    pub fn pop_this_turn(&mut self) -> Matrix<Output<K, V>, 1, 1> {
        Matrix::with_ctor((*self.data.size()).try_into().unwrap(),|p| {
            self.data[p].pop_this_turn()
        })
    }
}
