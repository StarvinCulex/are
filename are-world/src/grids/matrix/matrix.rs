pub struct Matrix<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> {
    elements: Vec<Element>,
    size: Coord<usize>,
}

#[allow(dead_code)]
impl<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    #[inline]
    pub fn with_ctor(
        size: &Coord<usize>,
        constructor: impl Fn(Option<Coord<usize>>) -> Element,
    ) -> Self {
        let mut instance = Self {
            elements: Vec::with_capacity(Self::get_alloc_size(*size)),
            size: *size,
        };
        for (pos, addr) in
            Shape::<CHUNK_WIDTH, CHUNK_HEIGHT>::rect(*size, Coord(0, 0) | *size - Coord(1, 1))
        {
            if addr != instance.elements.len() {
                for _ in instance.elements.len()..addr {
                    instance.elements.push(constructor(None));
                }
            }
            instance.elements.push(constructor(Some(pos)));
        }
        instance
    }
    #[inline]
    pub fn size(&self) -> &Coord<usize> {
        &self.size
    }
    #[inline]
    pub fn area<'m>(
        &'m self,
        shape: Shape<CHUNK_WIDTH, CHUNK_HEIGHT>,
    ) -> MatrixArea<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT> {
        MatrixArea::new(self, shape)
    }
    #[inline]
    pub fn area_mut<'m>(
        &'m mut self,
        shape: Shape<CHUNK_WIDTH, CHUNK_HEIGHT>,
    ) -> MatrixAreaMut<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT> {
        MatrixAreaMut::new(self, shape)
    }
    #[inline]
    fn normalize_pos<I>(size: Coord<usize>, pos: Coord<I>) -> Coord<usize>
    where
        I: std::convert::TryInto<isize>,
        <I as std::convert::TryInto<isize>>::Error: std::fmt::Debug,
    {
        pos.try_into()
            .unwrap()
            .reduce(size.try_into().unwrap(), isize::rem_euclid)
            .try_into()
            .unwrap()
    }
    #[inline]
    fn get_alloc_size(size: Coord<usize>) -> usize {
        (size / Coord(CHUNK_WIDTH, CHUNK_HEIGHT) * Coord(CHUNK_WIDTH, CHUNK_HEIGHT))
            .merge(|x, y| x * y)
    }
    #[inline]
    unsafe fn pos_at_unchecked(size: Coord<usize>, addr: usize) -> Coord<usize> {
        let chunk_address = addr / (CHUNK_WIDTH * CHUNK_HEIGHT);
        let grid_address = addr % (CHUNK_WIDTH * CHUNK_HEIGHT);
        let q = Coord(chunk_address % size.0, chunk_address / size.0);
        let r = Coord(grid_address % CHUNK_WIDTH, grid_address / CHUNK_WIDTH);
        q * Coord(CHUNK_WIDTH, CHUNK_HEIGHT) + r
    }
    #[inline]
    unsafe fn get_address_unchecked(size: Coord<usize>, at: Coord<usize>) -> usize {
        let q = at / Coord(CHUNK_WIDTH, CHUNK_HEIGHT);
        let r = at % Coord(CHUNK_WIDTH, CHUNK_HEIGHT);
        let chunk_address = q.0 + q.1 * size.0;
        let grid_address = r.0 + r.1 * CHUNK_WIDTH;
        chunk_address * CHUNK_WIDTH * CHUNK_HEIGHT + grid_address
    }
    #[inline]
    unsafe fn get_by_addr(&self, addr: usize) -> &Element {
        self.elements.get_unchecked(addr)
    }
    #[inline]
    unsafe fn get_by_addr_mut<'m>(&'m mut self, addr: usize) -> &'m mut Element {
        self.elements.get_unchecked_mut(addr)
    }
}

impl<I, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> std::ops::Index<Coord<I>>
    for Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    I: std::convert::TryInto<isize>,
    <I as std::convert::TryInto<isize>>::Error: std::fmt::Debug,
{
    type Output = Element;
    fn index(&self, index: Coord<I>) -> &Element {
        unsafe {
            self.get_by_addr(Self::get_address_unchecked(
                self.size,
                Self::normalize_pos(self.size, index),
            ))
        }
    }
}

impl<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> std::ops::IndexMut<Coord<usize>>
    for Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
{
    fn index_mut(&mut self, index: Coord<usize>) -> &mut Element {
        unsafe {
            self.get_by_addr_mut(Self::get_address_unchecked(
                self.size,
                Self::normalize_pos(self.size, index),
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

#[allow(dead_code)]
impl<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Element: Default,
{
    #[inline]
    pub fn new(size: &Coord<usize>) -> Self {
        Self::with_ctor(size, |_| Element::default())
    }
}
