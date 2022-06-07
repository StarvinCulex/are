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
    #[inline]
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

    #[inline]
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
    #[inline]
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
    #[inline]
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

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.accessor
            .next()
            .map(|(pos, addr)| (pos, unsafe { self.matrix.get_by_addr(addr) }))
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
    #[inline]
    fn len(&self) -> usize {
        self.accessor.len()
    }
}

pub struct IntoIterator<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> {
    addr: usize,
    count: usize,
    size: Coord<isize>,
    data: Vec<MaybeUninit<Element>>,
}

impl<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> std::iter::Iterator
    for IntoIterator<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    type Item = (Coord<isize>, Element);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if std::intrinsics::unlikely(self.len() == 0) {
            return None;
        }
        while !Matrix::<Element, CHUNK_WIDTH, CHUNK_HEIGHT>::is_initialized(self.size, self.addr) {
            self.addr += 1;
        }
        let value = unsafe { self.data.get_unchecked_mut(self.addr).assume_init_read() };
        let p =
            Matrix::<Element, CHUNK_WIDTH, CHUNK_HEIGHT>::pos_at_unchecked(self.size, self.addr);
        self.addr += 1;
        self.count += 1;
        Some((p, value))
    }
}

impl<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> Drop
    for IntoIterator<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    #[inline]
    fn drop(&mut self) {
        for _ in self.by_ref() {}
    }
}

impl<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> std::iter::ExactSizeIterator
    for IntoIterator<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    #[inline]
    fn len(&self) -> usize {
        self.size.0 as usize * self.size.1 as usize - self.count
    }
}

impl<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> std::iter::IntoIterator
    for Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    type Item = (Coord<isize>, Element);
    type IntoIter = IntoIterator<Element, CHUNK_WIDTH, CHUNK_HEIGHT>;

    #[inline]
    fn into_iter(mut self) -> Self::IntoIter {
        IntoIterator {
            addr: 0,
            count: 0,
            size: self.size,
            data: std::mem::take(&mut self.elements),
        }
    }
}
