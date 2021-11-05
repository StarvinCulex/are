#[derive(Clone)]
pub enum Shape<const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> {
    Rect {
        matrix_size: Coord<usize>,
        range: Coord<Interval<usize>>,
        iter_at: Coord<usize>,
        iter_addr: usize,
        jump_addr: usize,
        // 这个位置标志了不能无脑+1地址的位置
        jump_at: Coord<usize>,
        end_addr: usize,
    },
    Group {
        groups: Vec<Self>,
        iter_at: usize,
    },
    Zero,
}

#[allow(dead_code)]
impl<const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> Shape<CHUNK_WIDTH, CHUNK_HEIGHT> {
    #[inline]
    pub fn all(matrix_size: Coord<usize>) -> Self {
        if matrix_size.0 * matrix_size.1 == 0 {
            Shape::Zero
        } else {
            unsafe { Shape::rect_unchecked(matrix_size, Coord(0, 0) | matrix_size - Coord(1, 1)) }
        }
    }
    pub fn rect(matrix_size: Coord<usize>, range: Coord<Interval<usize>>) -> Self {
        let wrap_hori = range.0.from > range.0.to;
        let wrap_vert = range.1.from > range.1.to;
        match (wrap_hori, wrap_vert) {
            (false, false) => unsafe { Self::rect_unchecked(matrix_size, range) },
            (true, false) => unsafe {
                Self::rect_unchecked(matrix_size, Coord(Interval::new(0, range.0.to), range.1))
                    .union(Self::rect_unchecked(
                        matrix_size,
                        Coord(Interval::new(range.0.from, matrix_size.0 - 1), range.1),
                    ))
            },
            (false, true) => unsafe {
                Self::rect_unchecked(matrix_size, Coord(range.0, Interval::new(0, range.1.to)))
                    .union(Self::rect_unchecked(
                        matrix_size,
                        Coord(range.0, Interval::new(range.1.from, matrix_size.1 - 1)),
                    ))
            },
            (true, true) => unsafe {
                Self::rect_unchecked(matrix_size, Coord(0, 0) | range.to())
                    .union(Self::rect_unchecked(
                        matrix_size,
                        Coord(
                            Interval::new(range.0.from, matrix_size.0 - 1),
                            Interval::new(0, range.1.to),
                        ),
                    ))
                    .union(Self::rect_unchecked(
                        matrix_size,
                        Coord(
                            Interval::new(0, range.0.to),
                            Interval::new(range.1.from, matrix_size.1 - 1),
                        ),
                    ))
                    .union(Self::rect_unchecked(
                        matrix_size,
                        range.from() | matrix_size - Coord(1, 1),
                    ))
            },
        }
    }
    #[inline]
    pub unsafe fn rect_unchecked(matrix_size: Coord<usize>, range: Coord<Interval<usize>>) -> Self {
        let first_at = Coord(range.0.from, range.1.from);
        let end_at = Coord(range.0.to, range.1.to);
        let (jump_addr, jump_at) = Self::calc_next_jump(&matrix_size, &range, &first_at);
        Self::Rect {
            matrix_size,
            range,
            iter_at: first_at,
            iter_addr: Matrix::<(), CHUNK_WIDTH, CHUNK_HEIGHT>::get_address_unchecked(
                matrix_size,
                first_at,
            ),
            jump_addr,
            jump_at,
            end_addr: Matrix::<(), CHUNK_WIDTH, CHUNK_HEIGHT>::get_address_unchecked(
                matrix_size,
                end_at,
            ),
        }
    }
    #[inline]
    pub fn union(self, b: Self) -> Self {
        match self {
            Self::Zero => b,
            Self::Group {
                mut groups,
                iter_at,
            } => match b {
                Self::Zero => Self::Group { groups, iter_at },
                _ => {
                    groups.push(b);
                    Self::Group { groups, iter_at }
                }
            },
            _ => match b {
                Self::Zero => self,
                Self::Group {
                    mut groups,
                    iter_at,
                } => {
                    groups.push(self);
                    Self::Group { groups, iter_at }
                }
                _ => Self::Group {
                    groups: vec![self, b],
                    iter_at: 0,
                },
            },
        }
    }
    #[inline]
    fn calc_next_jump(
        matrix_size: &Coord<usize>,
        range: &Coord<Interval<usize>>,
        iter_at: &Coord<usize>,
    ) -> (usize, Coord<usize>) {
        assert!(range.0.from <= range.0.to && range.1.from <= range.1.to);
        assert!((Coord(0, 0) | *matrix_size).contains(&range));
        let chunk = *iter_at / Coord(CHUNK_WIDTH, CHUNK_HEIGHT);
        let range_chunk_border = range.to() / Coord(CHUNK_WIDTH, CHUNK_HEIGHT);
        let range_border_full =
            ((range.to() + Coord(1, 1)) % Coord(CHUNK_WIDTH, CHUNK_HEIGHT)).map(|x| x == 0);
        todo!()
    }
}

impl<const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> std::iter::Iterator
    for Shape<CHUNK_WIDTH, CHUNK_HEIGHT>
{
    type Item = (Coord<usize>, usize);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Shape::Zero => None,
            Shape::Group { groups, iter_at } => {
                while *iter_at < groups.len() {
                    let value = groups[*iter_at].next();
                    if value.is_some() {
                        return value;
                    }
                    *iter_at += 1;
                }
                None
            }
            Shape::Rect {
                matrix_size,
                range,
                iter_at,
                iter_addr,
                jump_addr,
                jump_at,
                end_addr,
            } => {
                todo!()
            }
        }
    }
}

impl<const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> std::iter::FusedIterator
    for Shape<CHUNK_WIDTH, CHUNK_HEIGHT>
{
}
