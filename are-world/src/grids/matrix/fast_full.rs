/// 特化只能用来扫整个矩阵
pub struct FastFull<const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> {
    addr: usize,
    length: usize,
    matrix_size: Coord<isize>,
}

impl<const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> FastFull<CHUNK_WIDTH, CHUNK_HEIGHT> {
    #[inline]
    fn new(matrix_size: Coord<isize>) -> Self {
        Self {
            addr: 0,
            length: (matrix_size.0 * matrix_size.1) as usize,
            matrix_size,
        }
    }
}

impl<const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> Accessor<CHUNK_WIDTH, CHUNK_HEIGHT>
    for FastFull<CHUNK_WIDTH, CHUNK_HEIGHT>
{
    #[inline]
    fn next(&mut self) -> Option<(Coord<isize>, usize)> {
        if unlikely(self.length == 0) {
            return None;
        }
        if unlikely(!Matrix::<(), CHUNK_WIDTH, CHUNK_HEIGHT>::is_initialized(self.matrix_size, self.addr)) {
            loop {
                self.addr += 1;
                if unlikely(Matrix::<(), CHUNK_WIDTH, CHUNK_HEIGHT>::is_initialized(self.matrix_size, self.addr)) {
                    break;
                }
            }
        }
        let p = Matrix::<(), CHUNK_WIDTH, CHUNK_HEIGHT>::pos_at_unchecked(self.matrix_size, self.addr);
        let ret = Some((p, self.addr));
        self.addr += 1;
        self.length -= 1;
        ret
    }

    #[inline]
    fn len(&self) -> usize {
        self.length
    }

    #[inline]
    fn super_area(&self) -> Coord<Interval<isize>> {
        Coord(0, 0) | (self.matrix_size - Coord(1, 1))
    }

    #[inline]
    fn contains(&self, pos: Coord<isize>) -> bool {
        true
    }

    #[inline]
    fn r#type(&self) -> &'static str {
        "MFastFull"
    }
}
