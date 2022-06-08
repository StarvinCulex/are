pub struct Fast<const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> {
    addr: usize,
    max_addr: usize,
    length: usize,
    area: Coord<Interval<isize>>,
    matrix_size: Coord<isize>,
}

impl<const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> Fast<CHUNK_WIDTH, CHUNK_HEIGHT> {
    #[inline]
    fn new(area: Coord<Interval<isize>>, matrix_size: Coord<isize>) -> Self {
        let a = measure_area(matrix_size, area);
        Self {
            addr: Matrix::<(), CHUNK_WIDTH, CHUNK_HEIGHT>::calc_address_unchecked(
                matrix_size,
                area.from(),
            ),
            max_addr: Matrix::<(), CHUNK_WIDTH, CHUNK_HEIGHT>::calc_address_unchecked(
                matrix_size,
                matrix_size - Coord(1, 1),
            ),
            length: (a.0 * a.1) as usize,
            area,
            matrix_size,
        }
    }
}

impl<const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> Accessor<CHUNK_WIDTH, CHUNK_HEIGHT>
    for Fast<CHUNK_WIDTH, CHUNK_HEIGHT>
{
    #[inline]
    fn next(&mut self) -> Option<(Coord<isize>, usize)> {
        // 待优化：最坏情况下可能为了扫四个点而扫过整个矩阵
        while self.length > 0 {
            let p = Matrix::<(), CHUNK_WIDTH, CHUNK_HEIGHT>::pos_at_unchecked(
                self.matrix_size,
                self.addr,
            );
            let addr = self.addr;
            // 由于 Matrix 是个甜甜圈，扫到最后应该回头扫最前面
            self.addr = if unlikely(addr == self.max_addr) {
                0
            } else {
                addr + 1
            };
            // Coord 的 partial_cmp 是先比较前者再比较后者
            // 所以对于 1x1 矩阵来说 Coord(1, 0) < Coord(1, 1)
            // 但 Coord(1, 0) 仍然是越界，所以这里分开比较
            // 但前面比较过 addr < max_addr，所以 p.1 不可能越界，不用比较了
            if likely(p.0 < self.matrix_size.0 && /* p.1 < self.matrix_size.1 && */ self.area.contains(&p)) {
                self.length -= 1;
                return Some((p, addr));
            }
        }
        None
    }

    #[inline]
    fn len(&self) -> usize {
        self.length
    }

    #[inline]
    fn super_area(&self) -> Coord<Interval<isize>> {
        self.area
    }

    #[inline]
    fn contains(&self, pos: Coord<isize>) -> bool {
        self.area.contains(&Matrix::<(), CHUNK_WIDTH, CHUNK_HEIGHT>::normalize_pos(self.matrix_size, pos))
    }

    #[inline]
    fn r#type(&self) -> &'static str {
        "MFast"
    }
}
