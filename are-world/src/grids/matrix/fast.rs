pub struct Fast<const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> {
    area: Coord<Interval<isize>>,
    matrix_size: Coord<isize>,
    len: usize,
    addr: usize,
    end: usize,
    at: Coord<isize>,
    inner: FastInner,
}

enum FastInner {}

impl<const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> Fast<CHUNK_WIDTH, CHUNK_HEIGHT> {
    #[inline]
    fn new(area: Coord<Interval<isize>>, matrix_size: Coord<isize>) -> Self {
        todo!()
    }
}

impl<const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> Accessor<CHUNK_WIDTH, CHUNK_HEIGHT>
    for Fast<CHUNK_WIDTH, CHUNK_HEIGHT>
{
    #[inline]
    fn next(&mut self) -> Option<(Coord<isize>, usize)> {
        if unlikely(self.len == 0) {
            return None;
        }
        self.len -= 1;

        if likely(self.addr != self.end) {
            let r = (self.at, self.addr);
            self.addr += 1;
            self.at = Matrix::<(), CHUNK_WIDTH, CHUNK_HEIGHT>::pos_at_unchecked(
                self.matrix_size,
                self.addr,
            );
            return Some(r);
        }

        match self.inner {};
        self.next_addr_end();

        let r = (self.at, self.addr);
        self.addr += 1;
        Some(r)
    }

    #[inline]
    fn len(&self) -> usize {
        self.len
    }

    #[inline]
    fn super_area(&self) -> Coord<Interval<isize>> {
        self.area
    }

    #[inline]
    fn contains(&self, pos: Coord<isize>) -> bool {
        self.area
            .contains(&Matrix::<(), CHUNK_WIDTH, CHUNK_HEIGHT>::normalize_pos(
                self.matrix_size,
                pos,
            ))
    }

    #[inline]
    fn r#type(&self) -> &'static str {
        "MFast"
    }
}

impl<const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> Fast<CHUNK_WIDTH, CHUNK_HEIGHT> {
    #[inline]
    fn next_addr_end(&mut self) {
        match self.inner {}
    }
}
