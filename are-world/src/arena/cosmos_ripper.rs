use crate::arena::defs::{Crd, CrdI};
use crate::grids::Coord;

pub struct CosmosRipper {
    pub chunk_size: Crd, // 消息所在区间大小
    pub padding: Crd, // 消息所在区间周围可操作范围大小
    pub bound_size: Crd, // 可操作的区间范围大小
    pub batch_size: Crd, // 每批有多少个
}

pub struct CosmosRipperBatch<'c> {
    ripper: &'c CosmosRipper,
    batch_offset: Crd,
    consumed: Crd,
}

impl CosmosRipper {
    pub fn new(plate_size: Crd, chunk_size: Crd, padding: Crd) -> Self {
        // 为了避免区间重合，两个 padding 里应该可以正好放下 n 个 chunk
        debug_assert!(padding.0 * 2 % chunk_size.0 == 0 && padding.1 * 2 % chunk_size.1 == 0);
        let bound_size = Coord(chunk_size.0 + 2 * padding.0, chunk_size.1 + 2 * padding.1);
        // 方便起见，要求 plate_size 必须是 chunk_size 的整数倍
        debug_assert!(plate_size.0 % bound_size.0 == 0 && plate_size.1 % bound_size.1 == 0);
        let batch_size = Coord(plate_size.0 / bound_size.0, plate_size.1 / bound_size.1);
        Self {
            chunk_size,
            padding,
            bound_size,
            batch_size,
        }
    }
    #[inline]
    pub fn with(&self, mut func: impl FnMut(&mut CosmosRipperBatch)) {
        let batch_count = Coord(self.bound_size.0 / self.chunk_size.0, self.bound_size.1 / self.chunk_size.1); // 分多少批执行
        for i in 0..batch_count.0 {
            for j in 0..batch_count.1 {
                let mut batch = CosmosRipperBatch {
                    ripper: self,
                    batch_offset: Coord(i, j),
                    consumed: Coord(0, 0),
                };
                func(&mut batch);
                drop(batch);
            }
        }
    }
}

impl<'c> Iterator for CosmosRipperBatch<'c> {
    type Item = (CrdI, CrdI); // (chunk, bound)
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.consumed.0 >= self.ripper.batch_size.0 {
            return None;
        }
        let left_top = Coord(
            self.consumed.0 * self.ripper.bound_size.0 + self.batch_offset.0 * self.ripper.chunk_size.0,
            self.consumed.1 * self.ripper.bound_size.1 + self.batch_offset.1 * self.ripper.chunk_size.1,
        );
        let bound = left_top | (left_top + self.ripper.bound_size - Coord(1, 1));
        let chunk = (left_top + self.ripper.padding) | (left_top + self.ripper.padding + self.ripper.chunk_size - Coord(1, 1));
        let slice = (chunk, bound);
        self.consumed.1 += 1;
        if self.consumed.1 >= self.ripper.batch_size.1 {
            self.consumed.1 = 0;
            self.consumed.0 += 1;
        }
        Some(slice)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.len();
        (size, Some(size))
    }
}

impl<'c> ExactSizeIterator for CosmosRipperBatch<'c> {
    #[inline]
    fn len(&self) -> usize {
        if self.consumed.0 >= self.ripper.batch_size.0 {
            return 0;
        }
        ((self.ripper.batch_size.0 - self.consumed.0) * self.ripper.batch_size.1 - self.consumed.1) as usize
    }
}

impl<'c> std::iter::FusedIterator for CosmosRipperBatch<'c> {}
