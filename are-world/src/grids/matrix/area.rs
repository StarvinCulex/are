//! by *StarvinCulex @2022/01/15*
use std::iter::Iterator as _;

use super::*;

use ::duplicate::duplicate;

duplicate! {
    [
        AreaType     IterType         reference(T)  ref_life(T, a)  try_derive_clone   ref_or_val(T);
        [ Area    ]  [ Iterator    ]  [ &    T ]    [ &'a     T ]   [ derive(Clone) ]  [ &T ];
        [ AreaMut ]  [ IteratorMut ]  [ &mut T ]    [ &'a mut T ]   [ derive(     ) ]  [  T ];
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
        self: ref_or_val([Self]),
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

//         AreaMut<'m> -> Area   <'m>
// &'a     AreaMut<'m> -> Area   <'a>
// &'a mut AreaMut<'m> -> AreaMut<'a>
duplicate! {
    [
        lifetimes   into_lifetime  IntoType     ref_life(T, a);
        [ 'm     ]  [ 'm ]         [ Area    ]  [         T ];
        [ 'm, 'a ]  [ 'a ]         [ Area    ]  [ &'a     T ];
        [ 'm, 'a ]  [ 'a ]         [ AreaMut ]  [ &'a mut T ];
    ]
impl<lifetimes, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    Into<IntoType<into_lifetime, Element, CHUNK_WIDTH, CHUNK_HEIGHT>> for ref_life([AreaMut<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT>], [a])
{
    #[inline]
    fn into(self) -> IntoType<into_lifetime, Element, CHUNK_WIDTH, CHUNK_HEIGHT> {
        IntoType {
            matrix: self.matrix,
            area: self.area,
        }
    }
}

}
