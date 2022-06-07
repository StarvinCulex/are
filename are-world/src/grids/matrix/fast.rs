pub struct Fast<const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> {
    addr: usize,
    count: usize,
    length: usize,
    area: Coord<Interval<isize>>,
    matrix_size: Coord<isize>,
}

impl<const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> Fast<CHUNK_WIDTH, CHUNK_HEIGHT> {
    fn new(area: Coord<Interval<isize>>, matrix_size: Coord<isize>) -> Self {
        let a = measure_area(matrix_size, area);
        Self {
            addr: Matrix::<(), CHUNK_WIDTH, CHUNK_HEIGHT>::calc_address_unchecked(
                matrix_size,
                area.from(),
            ),
            count: 0,
            length: (a.0 * a.1) as usize,
            area,
            matrix_size,
        }
    }
}

impl<const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> Accessor<CHUNK_WIDTH, CHUNK_HEIGHT>
    for Fast<CHUNK_WIDTH, CHUNK_HEIGHT>
{
    fn next(&mut self) -> Option<(Coord<isize>, usize)> {
        while self.count < self.length {
            let p = Matrix::<(), CHUNK_WIDTH, CHUNK_HEIGHT>::pos_at_unchecked(
                self.matrix_size,
                self.addr,
            );
            let addr = self.addr;
            self.addr += 1;
            if unlikely(self.addr == (self.matrix_size.0 * self.matrix_size.1) as usize) {
                self.addr = 0;
            }
            if likely(p < self.matrix_size && self.area.contains(&p)) {
                self.count += 1;
                return Some((p, addr));
            }
        }
        None
    }

    fn len(&self) -> usize {
        self.length
    }

    fn super_area(&self) -> Coord<Interval<isize>> {
        self.area
    }

    fn contains(&self, pos: Coord<isize>) -> bool {
        self.area.contains(&pos)
    }

    fn r#type(&self) -> &'static str {
        "MFast"
    }
}
