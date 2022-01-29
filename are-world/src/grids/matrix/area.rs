//! by *StarvinCulex @2022/01/15*
use std::iter::Iterator as _;

use super::*;

#[derive(Clone)]
pub struct Area<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> {
    pub matrix: &'m Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>,
    pub area: Coord<Interval<isize>>,
}

impl<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    Area<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    #[inline]
    pub fn scan(
        &self,
    ) -> Iterator<'m, Element, Scan<CHUNK_WIDTH, CHUNK_HEIGHT>, CHUNK_WIDTH, CHUNK_HEIGHT> {
        Iterator::new(self.matrix, Scan::new(self.matrix.size, self.area))
    }

    #[inline]
    pub fn size(&self) -> Coord<isize> {
        measure_area(*self.matrix.size(), self.area)
    }

    #[inline]
    pub fn map<U, F: FnMut(&Element) -> U>(&self, mut mapping: F) -> Matrix<U, 1, 1> {
        let elements: Vec<U> = self
            .scan()
            .map(|(_, ref_element)| mapping(ref_element))
            .collect();
        Matrix::with_data(
            &Coord(self.size().0 as usize, self.size().1 as usize),
            elements,
        )
        .unwrap()
    }
}

impl<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> std::iter::IntoIterator
    for Area<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    type Item = (Coord<isize>, &'m Element);
    type IntoIter =
        Iterator<'m, Element, Scan<CHUNK_WIDTH, CHUNK_HEIGHT>, CHUNK_WIDTH, CHUNK_HEIGHT>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.scan()
    }
}
