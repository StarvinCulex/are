// by *StarvinCulex @2021/11/14*

impl<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    #[inline]
    pub fn scan(
        &self,
        area: Coord<Interval<isize>>,
    ) -> Iterator<Element, Scan<CHUNK_WIDTH, CHUNK_HEIGHT>, CHUNK_WIDTH, CHUNK_HEIGHT> {
        Iterator::new(self, Scan::new(self.size, self.normalize_area(area)))
    }

    #[inline]
    pub fn area(
        &self,
        area: Coord<Interval<isize>>,
    ) -> Iterator<Element, impl Accessor<CHUNK_WIDTH, CHUNK_HEIGHT>, CHUNK_WIDTH, CHUNK_HEIGHT>
    {
        self.scan(area)
    }

    #[inline]
    pub fn iter(
        &self,
    ) -> Iterator<Element, impl Accessor<CHUNK_WIDTH, CHUNK_HEIGHT>, CHUNK_WIDTH, CHUNK_HEIGHT>
    {
        self.area(Coord(0, 0) | (*self.size() - Coord(1, 1)))
    }
}

pub struct Iterator<
    'm,
    Element,
    Access: Accessor<CHUNK_WIDTH, CHUNK_HEIGHT>,
    const CHUNK_WIDTH: usize,
    const CHUNK_HEIGHT: usize,
> {
    pub matrix: &'m Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>,
    accessor: Access,
}

impl<'m, Element, Access, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    Iterator<'m, Element, Access, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Access: Accessor<CHUNK_WIDTH, CHUNK_HEIGHT>,
{
    fn new(matrix: &'m Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>, accessor: Access) -> Self {
        Self { matrix, accessor }
    }
}

impl<'m, Element, Access, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> std::iter::Iterator
    for Iterator<'m, Element, Access, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Access: Accessor<CHUNK_WIDTH, CHUNK_HEIGHT>,
{
    type Item = (Coord<isize>, &'m Element);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((pos, addr)) = self.accessor.next() {
            Some((pos, unsafe { self.matrix.get_by_addr(addr) }))
        } else {
            None
        }
    }
}

impl<'m, Element, Access, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    std::iter::FusedIterator for Iterator<'m, Element, Access, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Access: Accessor<CHUNK_WIDTH, CHUNK_HEIGHT>,
{
}

impl<'m, Element, Access, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    std::iter::ExactSizeIterator for Iterator<'m, Element, Access, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Access: Accessor<CHUNK_WIDTH, CHUNK_HEIGHT>,
{
    fn len(&self) -> usize {
        self.accessor.len()
    }
}
