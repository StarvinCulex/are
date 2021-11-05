pub struct MatrixArea<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> {
    matrix: &'m Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>,
    shape: Shape<CHUNK_WIDTH, CHUNK_HEIGHT>,
}

impl<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    MatrixArea<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    #[inline]
    fn new(
        matrix: &'m Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>,
        shape: Shape<CHUNK_WIDTH, CHUNK_HEIGHT>,
    ) -> MatrixArea<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT> {
        MatrixArea { matrix, shape }
    }
}

impl<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> std::ops::Index<Coord<usize>>
    for MatrixArea<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    type Output = Element;
    fn index(&self, index: Coord<usize>) -> &Element {
        &self.matrix[index]
    }
}

impl<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> Iterator
    for MatrixArea<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    type Item = (&'m Element, Coord<usize>);
    fn next(&mut self) -> Option<(&'m Element, Coord<usize>)> {
        if let Some((pos, addr)) = self.shape.next() {
            unsafe { Some((&self.matrix.get_by_addr(addr), pos)) }
        } else {
            None
        }
    }
}

/**
 * MatrixSliceMut
 */
pub struct MatrixAreaMut<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> {
    matrix: &'m mut Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>,
    shape: Shape<CHUNK_WIDTH, CHUNK_HEIGHT>,
}

#[allow(dead_code)]
impl<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    MatrixAreaMut<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    #[inline]
    fn new(
        matrix: &'m mut Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>,
        shape: Shape<CHUNK_WIDTH, CHUNK_HEIGHT>,
    ) -> MatrixAreaMut<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT> {
        MatrixAreaMut { matrix, shape }
    }
}

impl<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> std::ops::Index<Coord<usize>>
    for MatrixAreaMut<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    type Output = Element;
    fn index(&self, index: Coord<usize>) -> &Element {
        &self.matrix[index]
    }
}

impl<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    std::ops::IndexMut<Coord<usize>> for MatrixAreaMut<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    fn index_mut(&mut self, index: Coord<usize>) -> &mut Element {
        &mut self.matrix[index]
    }
}
/*
impl<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> Iterator
    for MatrixAreaMut<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    type Item = (&'m mut Element, Coord<usize>);
    fn next(&mut self) -> Option<(&'m mut Element, Coord<usize>)> {
        if let Some((pos, addr)) = self.shape.next() {
            unsafe {
                Some((self.matrix.get_by_addr_mut(addr), pos)
            }
        } else {
            None
        }
    }
}
*/
