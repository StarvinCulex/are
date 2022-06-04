// by *StarvinCulex @2021/11/13*

use std::intrinsics::unlikely;
use std::iter::{IntoIterator as _, Iterator as _};
use std::mem::MaybeUninit;
use std::ops::Index;

use ::duplicate::duplicate;

/// 固定宽度和高度的矩阵。  
/// 通过[`Coord<isize>`]作为索引获得其中的值。  
#[derive(Debug, Default)]
pub struct Matrix<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> {
    elements: Vec<MaybeUninit<Element>>,
    size: Coord<isize>,
}

#[allow(dead_code)]
impl<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    /// 构造大小为参数`size`的矩阵。
    /// 矩阵中的元素由参数函数`constructor`决定：  
    /// - `constructor(Some(Coord(x, y)))`的值填充`(x, y)`对映的位置
    /// - 不使用的区域用`constructor(None)`的值填充
    ///  
    /// *`size.0`或`size.1`超过[`isize::MAX`]引发未定义行为。*
    pub fn with_ctor<Index: Into<usize>>(
        size: Coord<Index>,
        mut constructor: impl FnMut(Coord<isize>) -> Element,
    ) -> Self {
        let size: Coord<usize> = Coord(size.0.into(), size.1.into());
        let alloc_size = Self::calc_alloc_size::<usize>(size);
        let mut instance = Self {
            elements: Vec::with_capacity(alloc_size),
            size: Coord(size.0 as isize, size.1 as isize),
        };
        for i in 0..alloc_size {
            let pos = Self::pos_at_unchecked(instance.size, i);
            let contains = (Coord(0, 0) | (instance.size - Coord(1, 1))).contains(&pos);
            let new_element = if std::intrinsics::likely(contains) {
                MaybeUninit::new(constructor(pos))
            } else {
                MaybeUninit::uninit()
            };
            instance.elements.push(new_element);
        }
        instance
    }

    pub fn with_iter<
        Size: Into<usize>,
        Index: Into<isize>,
        Iter: std::iter::Iterator<Item = (Coord<Index>, Element)>,
    >(
        size: Coord<Size>,
        iter: Iter,
    ) -> Result<Self, String> {
        let size: Coord<usize> = Coord(size.0.into(), size.1.into());
        let alloc_size = Self::calc_alloc_size::<usize>(size);
        let mut instance = Self {
            elements: Vec::with_capacity(alloc_size),
            size: Coord(size.0 as isize, size.1 as isize),
        };
        for _ in 0..alloc_size {
            instance.elements.push(MaybeUninit::uninit());
        }
        let expected_range =
            Coord::with_intervals(Coord(0, 0), Coord(size.0 as isize, size.1 as isize));
        let mut exist = Matrix::<bool, 1, 1>::new(size);
        for (pos, e) in iter {
            let pos: Coord<isize> = Coord(pos.0.into(), pos.1.into());
            if unlikely(!expected_range.contains(&pos)) {
                return Err(format!("{} out of range", pos));
            }
            unsafe {
                let x = exist.get_by_addr_mut(Matrix::<bool, 1, 1>::calc_address_unchecked(
                    exist.size, pos,
                ));
                if unlikely(*x) {
                    return Err(format!("{} initialized more than once", pos));
                }
                *x = true;
                *instance
                    .elements
                    .get_unchecked_mut(Self::calc_address_unchecked(instance.size, pos)) =
                    MaybeUninit::new(e);
            }
        }
        for (p, &b) in exist.as_area().fast() {
            if !b {
                return Err(format!("{} not initialized", p));
            }
        }

        Ok(instance)
    }

    /// 返回矩阵的大小  
    /// 返回值`size`满足`size.0 > 0`且`size.1 > 0`
    #[inline]
    pub const fn size(&self) -> &Coord<isize> {
        &self.size
    }

    #[inline]
    pub fn normalize(&self, pos: Coord<isize>) -> Coord<isize> {
        Self::normalize_pos(self.size, pos)
    }

    #[inline]
    pub fn normalize_area(&self, area: Coord<Interval<isize>>) -> Coord<Interval<isize>> {
        self.normalize(area.from()) | self.normalize(area.to())
    }
}

impl<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    #[inline]
    pub fn area_mut<Index: Into<isize> + Ord>(
        &mut self,
        a: Coord<Interval<Index>>,
    ) -> AreaMut<Element, CHUNK_WIDTH, CHUNK_HEIGHT> {
        let area = self.normalize_area(
            Coord(a.0.from.into(), a.1.from.into()) | Coord(a.0.to.into(), a.1.to.into()),
        );
        AreaMut { matrix: self, area }
    }

    #[inline]
    pub fn as_area_mut(&mut self) -> AreaMut<Element, CHUNK_WIDTH, CHUNK_HEIGHT> {
        self.area_mut(Coord(0, 0) | (*self.size() - Coord(1, 1)))
    }

    #[inline]
    pub fn iter_mut(
        &mut self,
    ) -> <AreaMut<Element, CHUNK_WIDTH, CHUNK_HEIGHT> as std::iter::IntoIterator>::IntoIter {
        self.as_area_mut().into_iter()
    }
}

impl<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    #[inline]
    pub fn area<Index: Into<isize> + Ord>(
        &self,
        a: Coord<Interval<Index>>,
    ) -> Area<Element, CHUNK_WIDTH, CHUNK_HEIGHT> {
        let area = self.normalize_area(
            Coord(a.0.from.into(), a.1.from.into()) | Coord(a.0.to.into(), a.1.to.into()),
        );
        Area { matrix: self, area }
    }

    #[inline]
    pub fn as_area(&self) -> Area<Element, CHUNK_WIDTH, CHUNK_HEIGHT> {
        self.area(Coord(0, 0) | (*self.size() - Coord(1, 1)))
    }

    #[inline]
    pub fn iter(
        &self,
    ) -> <Area<Element, CHUNK_WIDTH, CHUNK_HEIGHT> as std::iter::IntoIterator>::IntoIter {
        self.as_area().into_iter()
    }
}

#[allow(dead_code)]
impl<Element> Matrix<Element, 1, 1> {
    #[inline]
    pub fn with_data(size: Coord<usize>, mut elements: Vec<Element>) -> Result<Self, ()> {
        if std::intrinsics::unlikely(elements.len() != size.0 * size.1) {
            Err(())
        } else {
            elements.shrink_to_fit();
            Ok(Self {
                size: size.try_into().unwrap(),
                elements: elements.into_iter().map(|x| MaybeUninit::new(x)).collect(),
            })
        }
    }

    #[inline]
    pub fn with_array2<const X: usize, const Y: usize>(elements: [[Element; X]; Y]) -> Self {
        let mut dim_reduct = Vec::with_capacity(X * Y);
        for row in elements {
            for e in row {
                dim_reduct.push(e);
            }
        }
        unsafe { Matrix::<_, 1, 1>::with_data(Coord(X, Y), dim_reduct).unwrap_unchecked() }
    }
}

impl<Element, const SIZE_X: usize, const SIZE_Y: usize> From<[[Element; SIZE_X]; SIZE_Y]>
    for Matrix<Element, 1, 1>
where
    Element: Default,
{
    fn from(elements: [[Element; SIZE_X]; SIZE_Y]) -> Self {
        Matrix::with_array2(elements)
    }
}

#[allow(dead_code)]
impl<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Element: Default,
{
    /// 构造大小为参数`size`的矩阵。  
    /// 矩阵的所有元素由`Element::default()`的结果填充。  
    #[inline]
    pub fn new<Index: Into<usize>>(size: Coord<Index>) -> Self {
        Self::with_ctor(size, |_| Element::default())
    }
}

impl<Index, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    std::ops::Index<Coord<Index>> for Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Index: Into<isize>,
{
    type Output = Element;
    fn index(&self, index: Coord<Index>) -> &Element {
        unsafe {
            self.get_by_addr(Self::calc_address_unchecked(
                self.size,
                self.normalize(Coord(index.0.into(), index.1.into())),
            ))
        }
    }
}

impl<Index, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    std::ops::IndexMut<Coord<Index>> for Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Index: Into<isize>,
{
    fn index_mut(&mut self, index: Coord<Index>) -> &mut Element {
        unsafe {
            self.get_by_addr_mut(Self::calc_address_unchecked(
                self.size,
                self.normalize(Coord(index.0.into(), index.1.into())),
            ))
        }
    }
}

#[allow(dead_code)]
impl<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Element: Clone,
{
    /// 构造大小为参数`size`的矩阵。  
    /// 矩阵中所有的元素由`element.clone()`的结果填充  
    ///   
    /// *`size.0`或`size.1`超过[`isize::MAX`]引发未定义行为。*
    #[inline]
    pub fn with_fill(size: Coord<usize>, element: &Element) -> Self {
        Self::with_ctor(size, |_| element.clone())
    }
}

impl<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> Clone
    for Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Element: Clone,
{
    fn clone(&self) -> Self {
        Self {
            elements: (0..self.elements.len())
                .zip(self.elements.iter())
                .map(|(addr, element)| {
                    if std::intrinsics::likely(Self::is_initialized(self.size, addr)) {
                        MaybeUninit::new(unsafe { element.assume_init_ref() }.clone())
                    } else {
                        MaybeUninit::uninit()
                    }
                })
                .collect(),
            size: self.size,
        }
    }
}

impl<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> PartialEq
    for Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Element: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }

        for (pos, element) in self.iter() {
            if element != &other[pos] {
                return false;
            }
        }
        true
    }
}

impl<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> Eq
    for Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Element: PartialEq + Eq,
{
}

impl<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    Into<AreaMut<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT>>
    for &'m mut Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    #[inline]
    fn into(self) -> AreaMut<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT> {
        self.as_area_mut()
    }
}

impl<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    Into<Area<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT>>
    for &'m Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    #[inline]
    fn into(self) -> Area<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT> {
        self.as_area()
    }
}

impl<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> Drop
    for Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    #[inline]
    fn drop(&mut self) {
        for i in (0..self.elements.len()).filter(|&i| Self::is_initialized(self.size, i)) {
            unsafe { self.elements.get_unchecked_mut(i).assume_init_drop() }
        }
    }
}

// private

impl<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    #[inline]
    const fn is_initialized(size: Coord<isize>, addr: usize) -> bool {
        (CHUNK_WIDTH == 1 && CHUNK_HEIGHT == 1) || {
            let c = Self::pos_at_unchecked(size, addr);
            c.0 < size.0 && c.1 < size.1
        }
    }

    #[inline]
    const fn calc_chunk_size(size: Coord<usize>) -> Coord<usize> {
        let chunk_row_count = (size.0 - 1) / CHUNK_WIDTH + 1;
        let chunk_col_count = (size.1 - 1) / CHUNK_HEIGHT + 1;
        Coord(chunk_row_count, chunk_col_count)
    }

    /// 计算`size`大小的矩阵需要分配多长的数组才能存下
    #[inline]
    const fn calc_alloc_size<Index: Into<usize>>(size: Coord<usize>) -> usize {
        let chunk_size = Self::calc_chunk_size(size);
        let chunk_count = chunk_size.0 * chunk_size.1;
        CHUNK_WIDTH * CHUNK_HEIGHT * chunk_count
    }

    /// 计算在`size`大小的矩阵中，偏移量`addr`对应的位置  
    /// 结果可能不在`size`大小矩阵所表示的区域里  
    /// `size`任何一个维度必须非负  
    #[inline]
    const fn pos_at_unchecked(size: Coord<isize>, addr: usize) -> Coord<isize> {
        let Coord(chunk_row_count, _) =
            Self::calc_chunk_size(Coord(size.0 as usize, size.1 as usize));
        let chunk_address = addr / (CHUNK_WIDTH * CHUNK_HEIGHT);
        let grid_address = addr % (CHUNK_WIDTH * CHUNK_HEIGHT);
        let q = Coord(
            chunk_address % chunk_row_count,
            chunk_address / chunk_row_count,
        );
        let r = Coord(grid_address % CHUNK_WIDTH, grid_address / CHUNK_WIDTH);
        Coord(
            (q.0 * CHUNK_WIDTH + r.0) as isize,
            (q.1 * CHUNK_HEIGHT + r.1) as isize,
        )
    }

    /// 计算在`size`大小矩阵中，位置`at`对应的偏移量  
    /// `size`、`at`任何一个维度必须非负  
    #[inline]
    const fn calc_address_unchecked(size: Coord<isize>, at: Coord<isize>) -> usize {
        let Coord(chunk_row_count, _) =
            Self::calc_chunk_size(Coord(size.0 as usize, size.1 as usize));
        let at0 = at.0 as usize;
        let at1 = at.1 as usize;
        let q = Coord(at0 / CHUNK_WIDTH, at1 / CHUNK_HEIGHT);
        let r = Coord(at0 % CHUNK_WIDTH, at1 % CHUNK_HEIGHT);
        let chunk_address = q.0 + q.1 * chunk_row_count;
        let grid_address = r.0 + r.1 * CHUNK_WIDTH;
        chunk_address * CHUNK_WIDTH * CHUNK_HEIGHT + grid_address
    }

    #[inline]
    unsafe fn get_by_addr(&self, addr: usize) -> &Element {
        self.elements.get_unchecked(addr).assume_init_ref()
    }

    #[inline]
    unsafe fn get_by_addr_mut(&mut self, addr: usize) -> &mut Element {
        self.elements.get_unchecked_mut(addr).assume_init_mut()
    }

    #[inline]
    pub fn normalize_pos(size: Coord<isize>, pos: Coord<isize>) -> Coord<isize> {
        pos.reduce(size, isize::rem_euclid)
    }

    #[inline]
    pub fn normalize_area_with(
        size: Coord<isize>,
        area: Coord<Interval<isize>>,
    ) -> Coord<Interval<isize>> {
        Self::normalize_pos(size, area.from()) | Self::normalize_pos(size, area.to())
    }
}

// tests
#[cfg(test)]
fn test_sub<const CW: usize, const CH: usize>() {
    let size = Coord(5usize, 5);
    let matrix = Matrix::<String, CW, CH>::with_ctor(size, |pos| pos.to_string());

    assert_eq!(*matrix.size(), Coord(5isize, 5isize));

    for j in 0..5isize {
        for i in 0..5isize {
            let expected = Coord(i, j).to_string();
            let value = &matrix[Coord(i, j)];
            assert_eq!(&expected, value);
        }
    }
    for (pos, value) in matrix.as_area().scan() {
        assert!(Coord(0, 0) <= pos && pos < matrix.size);
        assert_eq!(*value, pos.to_string());
    }
    for (pos, value) in matrix.iter() {
        assert!(Coord(0, 0) <= pos && pos < matrix.size);
        assert_eq!(*value, pos.to_string());
    }
}

#[cfg(test)]
fn test_with_iter<const CW: usize, const CH: usize>() {
    let size = Coord(89usize, 97);
    let matrix1 = Matrix::<String, 1, 1>::with_ctor(size, |pos| pos.to_string());
    let matrix2 = Matrix::<String, CW, CH>::with_iter(
        size,
        matrix1.as_area().fast().map(|(a, b)| (a, b.clone())),
    )
    .unwrap();
    for j in 0..size.1 {
        for i in 0..size.0 {
            let p = Coord(i as isize, j as isize);
            assert_eq!(matrix1[p], matrix2[p]);
        }
    }
}

#[cfg(test)]
#[test]
fn test() {
    test_sub::<1, 1>();
    test_sub::<2, 2>();
    test_with_iter::<1, 1>();
    test_with_iter::<29, 2>();
    test_with_iter::<13, 17>();
    // test_map();
}
