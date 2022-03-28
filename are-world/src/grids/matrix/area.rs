//! by *StarvinCulex @2022/01/15*
use std::iter::Iterator as _;

use super::*;

use ::duplicate::duplicate;

duplicate! {
    [
        try_derive_clone   AreaType     IterType         reference(T)  ref_life(T, a)  clone_or_move(T);
        [ derive(Clone) ]  [ Area ]     [ Iterator ]     [ &T ]        [ &'a T ]       [ &T ];
        [ derive() ]       [ AreaMut ]  [ IteratorMut ]  [ &mut T ]    [ &'a mut T ]   [ T ];
    ]

// mutable reference can't be cloned
#[try_derive_clone]
pub struct AreaType<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> {
    pub matrix: ref_life([Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>], [m]),
    pub area: Coord<Interval<isize>>,
}

impl<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    AreaType<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    // scan() will consume AreaMut but not Area, as Area derives Clone while AreaMut can't
    // another available choice is to make IteratorMut returned from scan() not outlive AreaMut
    #[inline]
    pub fn scan(
        self: clone_or_move([Self]),
    ) -> IterType<'m, Element, Scan<CHUNK_WIDTH, CHUNK_HEIGHT>, CHUNK_WIDTH, CHUNK_HEIGHT> {
        IterType::new(self.matrix, Scan::new(self.matrix.size, self.area))
    }

    #[inline]
    pub fn size(&self) -> Coord<isize> {
        measure_area(*self.matrix.size(), self.area)
    }
}

impl<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> std::iter::IntoIterator
    for AreaType<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    type Item = (Coord<isize>, ref_life([Element], [m]));
    type IntoIter =
        IterType<'m, Element, Scan<CHUNK_WIDTH, CHUNK_HEIGHT>, CHUNK_WIDTH, CHUNK_HEIGHT>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.scan()
    }
}

}

impl<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    Area<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
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

impl<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    AreaMut<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    // map() behaves the same, as the name `map` indicates it should not be mutating original data
    #[inline]
    pub fn map<U, F: FnMut(&Element) -> U>(&self, mapping: F) -> Matrix<U, 1, 1> {
        let area: Area<Element, CHUNK_WIDTH, CHUNK_HEIGHT> = self.into();
        area.map(mapping)
    }
}

impl<'m, 'a, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    Into<Area<'a, Element, CHUNK_WIDTH, CHUNK_HEIGHT>> for &'a AreaMut<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    #[inline]
    fn into(self) -> Area<'a, Element, CHUNK_WIDTH, CHUNK_HEIGHT> {
        Area {
            matrix: self.matrix,
            area: self.area,
        }
    }
}

impl<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    Into<Area<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT>> for AreaMut<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    #[inline]
    fn into(self) -> Area<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT> {
        Area {
            matrix: self.matrix,
            area: self.area,
        }
    }
}
