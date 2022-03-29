// by *StarvinCulex @2021/11/14*

duplicate! {
    [
        IterType         reference(T)  ref_life(T, a)  get_by_addr_fn;
        [ Iterator    ]  [ &    T ]    [ &'a     T ]   [ get_by_addr     ];
        [ IteratorMut ]  [ &mut T ]    [ &'a mut T ]   [ get_by_addr_mut ];
    ]

pub struct IterType<
    'm,
    Element,
    Access: Accessor<CHUNK_WIDTH, CHUNK_HEIGHT>,
    const CHUNK_WIDTH: usize,
    const CHUNK_HEIGHT: usize,
> {
    pub matrix: ref_life([Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>], [m]),
    accessor: Access,
}

impl<'m, Element, Access, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    IterType<'m, Element, Access, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Access: Accessor<CHUNK_WIDTH, CHUNK_HEIGHT>,
{
    fn new(matrix: ref_life([Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>], [m]), accessor: Access) -> Self {
        Self { matrix, accessor }
    }
}

impl<'m, Element, Access, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> std::iter::Iterator
    for IterType<'m, Element, Access, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Access: Accessor<CHUNK_WIDTH, CHUNK_HEIGHT>,
{
    type Item = (Coord<isize>, ref_life([Element], [m]));

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((pos, addr)) = self.accessor.next() {
            // extend the lifetime to 'm (the lifetime of Matrix) while preserving borrow checker working
            Some((pos, unsafe { std::mem::transmute(self.matrix.get_by_addr_fn(addr)) } ))
        } else {
            None
        }
    }
}

impl<'m, Element, Access, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    std::iter::FusedIterator for IterType<'m, Element, Access, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Access: Accessor<CHUNK_WIDTH, CHUNK_HEIGHT>,
{
}

impl<'m, Element, Access, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    std::iter::ExactSizeIterator for IterType<'m, Element, Access, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Access: Accessor<CHUNK_WIDTH, CHUNK_HEIGHT>,
{
    fn len(&self) -> usize {
        self.accessor.len()
    }
}

}
