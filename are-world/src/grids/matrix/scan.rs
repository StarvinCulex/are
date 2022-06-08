// by *StarvinCulex @2021/11/21*
pub struct Scan<const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> {
    at: Coord<isize>,
    length: usize,

    area: Coord<Interval<isize>>,

    matrix_size: Coord<isize>,
}

impl<const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> Scan<CHUNK_WIDTH, CHUNK_HEIGHT> {
    /// ```rust
    /// assert!(Coord(0, 0) < from && from <= matrix_size)
    /// assert!(Coord(0, 0) < to && to <= matrix_size)
    #[inline]
    fn new(matrix_size: Coord<isize>, area: Coord<Interval<isize>>) -> Self {
        let mut instance = Self {
            at: area.from(),
            length: 0,
            area,
            matrix_size,
        };
        let width = if unlikely(instance.wrap_x()) {
            matrix_size.0 - area.0.from + area.0.to + 1
        } else {
            area.0.to - area.0.from + 1
        };
        let height = if unlikely(instance.wrap_y()) {
            matrix_size.1 - area.1.from + area.1.to + 1
        } else {
            area.1.to - area.1.from + 1
        };
        instance.length = (width * height) as usize;
        instance
    }

    #[inline]
    fn wrap_x(&self) -> bool {
        self.area.0.to < self.area.0.from
    }

    #[inline]
    fn wrap_y(&self) -> bool {
        self.area.1.to < self.area.1.from
    }
}

impl<const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> Accessor<CHUNK_WIDTH, CHUNK_HEIGHT>
    for Scan<CHUNK_WIDTH, CHUNK_HEIGHT>
{
    #[inline]
    fn next(&mut self) -> Option<(Coord<isize>, usize)> {
        if unlikely(self.length == 0) {
            None
        } else {
            let at = self.at;
            let addr = Matrix::<(), CHUNK_WIDTH, CHUNK_HEIGHT>::calc_address_unchecked(
                self.matrix_size,
                at,
            );

            self.length -= 1;
            let next_line = at.0 == self.area.0.to;
            if unlikely(next_line) {
                let y = if unlikely(at.1 + 1 == self.matrix_size.1) {
                    0
                } else {
                    at.1 + 1
                };
                self.at = Coord(self.area.0.from, y);
            } else {
                let x = if unlikely(at.0 + 1 == self.matrix_size.0) {
                    0
                } else {
                    at.0 + 1
                };
                self.at.0 = x;
            }

            Some((at, addr))
        }
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
        "MScan"
    }
}
