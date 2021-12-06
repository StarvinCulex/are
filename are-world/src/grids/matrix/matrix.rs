// by *StarvinCulex @2021/11/13*

/// 固定宽度和高度的矩阵。  
/// 通过[`Coord<isize>`]作为索引获得其中的值。  
pub struct Matrix<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> {
    elements: Vec<Element>,
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
    #[inline]
    pub fn with_ctor(
        size: &Coord<usize>,
        mut constructor: impl FnMut(Option<Coord<isize>>) -> Element,
    ) -> Self {
        let alloc_size = Self::calc_alloc_size(*size);
        let mut instance = Self {
            elements: Vec::with_capacity(alloc_size),
            size: Coord(size.0 as isize, size.1 as isize),
        };
        for i in 0..alloc_size {
            let pos = unsafe { Self::pos_at_unchecked(instance.size, i) };
            let contains = (Coord(0, 0) | (instance.size - Coord(1, 1))).contains(&pos);
            let verified_pos = if contains { Some(pos) } else { None };
            let new_element = constructor(verified_pos);
            instance.elements.push(new_element);
        }
        instance
    }

    #[inline]
    pub fn with<const X: usize, const Y: usize>(
        mut elements: [[Element; X]; Y],
        none: impl Fn() -> Element,
    ) -> Self {
        Self::with_ctor(&Coord(X, Y), |opt_pos| {
            if let Some(pos) = opt_pos {
                if pos >= Coord(0, 0) && pos < Coord(X as isize, Y as isize) {
                    return std::mem::replace(
                        &mut elements[pos.1 as usize][pos.0 as usize],
                        none(),
                    );
                }
            }
            none()
        })
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

#[allow(dead_code)]
impl<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Element: Default,
{
    /// 构造大小为参数`size`的矩阵。  
    /// 矩阵的所有元素由`Element::default()`的结果填充。  
    #[inline]
    pub fn new(size: &Coord<usize>) -> Self {
        Self::with_ctor(size, |_| Element::default())
    }
}

impl<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> std::ops::Index<Coord<isize>>
    for Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    type Output = Element;
    fn index(&self, index: Coord<isize>) -> &Element {
        unsafe {
            self.get_by_addr(Self::calc_address_unchecked(
                self.size,
                self.normalize(index),
            ))
        }
    }
}

impl<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> std::ops::IndexMut<Coord<isize>>
    for Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    fn index_mut(&mut self, index: Coord<isize>) -> &mut Element {
        unsafe {
            self.get_by_addr_mut(Self::calc_address_unchecked(
                self.size,
                self.normalize(index),
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
    pub fn with_fill(size: &Coord<usize>, element: &Element) -> Self {
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
            elements: self.elements.clone(),
            size: self.size,
        }
    }
}

// private

impl<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    #[inline]
    const fn calc_chunk_size(size: Coord<usize>) -> Coord<usize> {
        let chunk_row_count = (size.0 - 1) / CHUNK_WIDTH + 1;
        let chunk_col_count = (size.1 - 1) / CHUNK_HEIGHT + 1;
        Coord(chunk_row_count, chunk_col_count)
    }

    /// 计算`size`大小的矩阵需要分配多长的数组才能存下
    #[inline]
    const fn calc_alloc_size(size: Coord<usize>) -> usize {
        let chunk_size = Self::calc_chunk_size(size);
        let chunk_count = chunk_size.0 * chunk_size.1;
        CHUNK_WIDTH * CHUNK_HEIGHT * chunk_count
    }

    /// 计算在`size`大小的矩阵中，偏移量`addr`对应的位置  
    /// 结果可能不在`size`大小矩阵所表示的区域里  
    /// `size`任何一个维度必须非负  
    #[inline]
    const unsafe fn pos_at_unchecked(size: Coord<isize>, addr: usize) -> Coord<isize> {
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
    const unsafe fn calc_address_unchecked(size: Coord<isize>, at: Coord<isize>) -> usize {
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
        self.elements.get_unchecked(addr)
    }

    #[inline]
    unsafe fn get_by_addr_mut(&mut self, addr: usize) -> &mut Element {
        self.elements.get_unchecked_mut(addr)
    }

    #[inline]
    fn normalize_pos(size: Coord<isize>, pos: Coord<isize>) -> Coord<isize> {
        pos.reduce(size, isize::rem_euclid)
    }
}

// tests
#[cfg(test)]
fn test_sub<const CW: usize, const CH: usize>() {
    let ctor = |opt_pos: Option<Coord<isize>>| {
        if let Some(pos) = opt_pos {
            pos.to_string()
        } else {
            "".to_string()
        }
    };
    let size = Coord(5, 5);
    let mut matrix = Matrix::<String, CW, CH>::with_ctor(&size, ctor);

    assert_eq!(*matrix.size(), Coord(5isize, 5isize));

    for j in 0..5 {
        for i in 0..5 {
            let expected = ctor(Some(Coord(i, j)));
            let value = &matrix[Coord(i, j)];
            assert_eq!(&expected, value);
        }
    }

    for (pos, value) in matrix.iter() {
        assert!(Coord(0, 0) <= pos && pos < matrix.size);
        assert_eq!(*value, ctor(Some(pos)));
    }
}

#[cfg(test)]
#[test]
fn test() {
    test_sub::<1, 1>();
    test_sub::<2, 2>();
}
