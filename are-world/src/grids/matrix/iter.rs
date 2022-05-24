// by *StarvinCulex @2021/11/14*

pub struct IteratorMut<
    'm,
    Element,
    Access: Accessor<CHUNK_WIDTH, CHUNK_HEIGHT>,
    const CHUNK_WIDTH: usize,
    const CHUNK_HEIGHT: usize,
> {
    pub matrix: &'m mut Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>,
    accessor: Access,
}

impl<'m, Element, Access, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    IteratorMut<'m, Element, Access, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Access: Accessor<CHUNK_WIDTH, CHUNK_HEIGHT>,
{
    fn new(matrix: &'m mut Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>, accessor: Access) -> Self {
        Self { matrix, accessor }
    }
}

impl<'m, Element, Access, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> std::iter::Iterator
    for IteratorMut<'m, Element, Access, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Access: Accessor<CHUNK_WIDTH, CHUNK_HEIGHT>,
{
    type Item = (Coord<isize>, &'m mut Element);

    fn next(&mut self) -> Option<Self::Item> {
        self.accessor.next().map(|(pos, addr)| {
            // extend the lifetime to 'm (the lifetime of Matrix) while preserving borrow checker working
            (pos, unsafe {
                std::mem::transmute(self.matrix.get_by_addr_mut(addr))
            })
        })
    }
}

impl<'m, Element, Access, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    std::iter::FusedIterator for IteratorMut<'m, Element, Access, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Access: Accessor<CHUNK_WIDTH, CHUNK_HEIGHT>,
{
}

impl<'m, Element, Access, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    std::iter::ExactSizeIterator for IteratorMut<'m, Element, Access, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Access: Accessor<CHUNK_WIDTH, CHUNK_HEIGHT>,
{
    fn len(&self) -> usize {
        self.accessor.len()
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
        self.accessor.next().map(|(pos, addr)| {
            (pos, unsafe {
                self.matrix.get_by_addr(addr)
            })
        })
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
