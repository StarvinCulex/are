//! by *StarvinCulex @2021/10/24*

use super::{coord::Coord, interval::Interval};

impl<T> std::ops::BitOr for Coord<T>
where
    T: Ord,
{
    type Output = Coord<Interval<T>>;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Coord(Interval::new(self.0, rhs.0), Interval::new(self.1, rhs.1))
    }
}

#[allow(dead_code)]
impl<T> Coord<Interval<T>>
where
    T: Ord,
{
    /// ```rust
    /// return Coord(self.0.from, self.1.from);
    #[inline]
    pub fn from(self) -> Coord<T> {
        Coord(self.0.from, self.1.from)
    }
    /// ```rust
    /// return Coord(self.0.to, self.1.to);
    #[inline]
    pub fn to(self) -> Coord<T> {
        Coord(self.0.to, self.1.to)
    }
    /// 判断`point`是否属于`self`表示的范围
    #[inline]
    pub fn contains(&self, point: &Coord<T>) -> bool {
        self.0.contains(&point.0) && self.1.contains(&point.1)
    }
}

#[allow(dead_code)]
impl<T> Coord<Interval<T>>
where
    T: Ord + std::ops::Add + Clone,
    <T as std::ops::Add>::Output: Ord,
{
    /// | 原值 | 返回值 |
    /// |:---:|:-----:|
    /// |`([p->q], [r->s])`|`([p+rhs.0->q+rhs.0], [r+rhs.1->s+rhs.1])`|
    #[inline]
    pub fn offset(self, rhs: Coord<T>) -> Coord<Interval<<T as std::ops::Add>::Output>> {
        self.reduce(rhs, Interval::offset)
    }
}

#[allow(dead_code)]
impl<T> Coord<Interval<T>>
where
    T: Ord + std::ops::Add<T> + std::ops::Sub<T> + Clone,
    <T as std::ops::Sub<T>>::Output: Into<<T as std::ops::Add<T>>::Output>,
    <T as std::ops::Add>::Output: std::cmp::Ord,
{
    #[inline]
    pub fn expand(self, rhs: Coord<T>) -> Coord<Interval<<T as std::ops::Add>::Output>> {
        self.reduce(rhs, Interval::offset)
    }
}
