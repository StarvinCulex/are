//! by *StarvinCulex @2022/01/15*

use super::*;

#[derive(Clone)]
pub struct Area<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> {
    pub matrix: &'m Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>,
    pub area: Coord<Interval<isize>>,
}

impl<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    Area<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    // scan() will consume AreaMut but not Area, as Area derives Clone while AreaMut can't
    // another available choice is to make IteratorMut returned from scan() not outlive AreaMut
    #[inline]
    pub fn scan(
        &self,
    ) -> Iterator<'m, Element, Scan<CHUNK_WIDTH, CHUNK_HEIGHT>, CHUNK_WIDTH, CHUNK_HEIGHT> {
        Iterator::new(self.matrix, Scan::new(self.matrix.size, self.area))
    }

    #[deprecated]
    #[inline]
    pub fn fast(
        &self,
    ) -> Iterator<'m, Element, Fast<CHUNK_WIDTH, CHUNK_HEIGHT>, CHUNK_WIDTH, CHUNK_HEIGHT> {
        Iterator::new(self.matrix, Fast::new(self.area, self.matrix.size))
    }

    #[inline]
    pub fn iter(&self) -> impl std::iter::Iterator<Item = (Coord<isize>, &'m Element)> {
        self.scan()
    }

    #[inline]
    pub fn size(&self) -> Coord<isize> {
        measure_area(*self.matrix.size(), self.area)
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

pub struct AreaMut<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> {
    pub matrix: &'m mut Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>,
    pub area: Coord<Interval<isize>>,
}

impl<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    AreaMut<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    // scan() will consume AreaMut but not Area, as Area derives Clone while AreaMut can't
    // another available choice is to make IteratorMut returned from scan() not outlive AreaMut
    #[inline]
    pub fn scan(
        self,
    ) -> IteratorMut<'m, Element, Scan<CHUNK_WIDTH, CHUNK_HEIGHT>, CHUNK_WIDTH, CHUNK_HEIGHT> {
        IteratorMut::new(self.matrix, Scan::new(self.matrix.size, self.area))
    }

    #[deprecated]
    #[inline]
    pub fn fast(
        self,
    ) -> IteratorMut<'m, Element, Fast<CHUNK_WIDTH, CHUNK_HEIGHT>, CHUNK_WIDTH, CHUNK_HEIGHT> {
        IteratorMut::new(self.matrix, Fast::new(self.area, self.matrix.size))
    }

    #[inline]
    pub fn iter(self) -> impl std::iter::Iterator<Item = (Coord<isize>, &'m mut Element)> {
        self.scan()
    }

    #[inline]
    pub fn size(&self) -> Coord<isize> {
        measure_area(*self.matrix.size(), self.area)
    }
}

impl<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> std::iter::IntoIterator
    for AreaMut<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    type Item = (Coord<isize>, &'m mut Element);
    type IntoIter =
        IteratorMut<'m, Element, Scan<CHUNK_WIDTH, CHUNK_HEIGHT>, CHUNK_WIDTH, CHUNK_HEIGHT>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.scan()
    }
}

//

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
            Coord(self.size().0 as usize, self.size().1 as usize),
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
    From<ref_life([AreaMut<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT>], [a])> for IntoType<into_lifetime, Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    #[inline]
    fn from(from: ref_life([AreaMut<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT>], [a])) -> Self {
        IntoType {
            matrix: from.matrix,
            area: from.area,
        }
    }
}

}
