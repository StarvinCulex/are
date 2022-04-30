//! by *StarvinCulex @2021/10/24*

use std::fmt::Formatter;

/// 表示一维范围
///
/// 包含字段`from`和`to`两个`<T>`类型的字段
///
/// 应该使用[`Interval::new`]方法构造
///
/// 当`from <= to`时：
/// - 表示大于等于`from`且小于等于`to`的范围
/// - 区间表示：**\[from, to]**
///
/// 当`from > to`时：
/// - 表示小于等于`from`或大于等于`to`的范围
/// - 区间表示：**(-∞, from] ∪ [to, +∞)**
use serde::{Deserialize, Serialize};

#[derive(Hash, Debug, Serialize, Deserialize)]
pub struct Interval<T>
where
    T: Ord,
{
    pub from: T,
    pub to: T,
}

#[allow(dead_code)]
impl<T> Interval<T>
where
    T: Ord,
{
    /// ```rust
    /// return Interval{from, to};
    #[inline]
    pub const fn new(from: T, to: T) -> Interval<T> {
        Interval { from, to }
    }

    /// 判断`point`是否属于`self`表示的范围
    pub fn contains(&self, point: &T) -> bool {
        let greater_than_from = point >= &self.from;
        let less_than_to = point <= &self.to;
        let closed_interval = self.from <= self.to;

        // closed_interval ^ greater_than_from ^ less_than_to
        if closed_interval {
            greater_than_from && less_than_to
        } else {
            greater_than_from || less_than_to
        }
    }

    /// 判断`interval`是否属于`self`表示的范围
    pub fn contains_interval(&self, interval: &Self, size: T) -> bool where T: std::ops::Add<T> + From<bool> + Copy, <T as std::ops::Add<T>>::Output: Eq {
        let self_closed_interval = self.from <= self.to;
        let other_closed_interval = interval.from <= interval.to;

        if self_closed_interval {
            other_closed_interval && self.from <= interval.from && interval.to <= self.to || self.from == size.0 && self.to.add(true.into()) == size.add(false.into())
        } else if other_closed_interval {
            interval.to <= self.from || self.to <= interval.from || self.to.add(true.into()) == self.from.add(false.into())
        } else {
            interval.from <= self.from && self.to <= interval.to
        }
    }

    /// 返回`Interval{from: self.to, to: self.from}`
    #[inline]
    pub fn inverse(self) -> Self {
        Self::new(self.to, self.from)
    }
}

#[allow(dead_code)]
impl<T> Interval<T>
where
    T: Ord + std::ops::Add + Clone,
    <T as std::ops::Add>::Output: Ord,
{
    /// 将`self.from`和`self.to`都加上`rhs`得到的值
    #[inline]
    pub fn offset(self, rhs: T) -> Interval<<T as std::ops::Add>::Output> {
        Interval::new(self.from + rhs.clone(), self.to + rhs)
    }
}

#[allow(dead_code)]
impl<T> Interval<T>
where
    T: Ord + std::ops::Add<T> + std::ops::Sub<T> + Clone,
    <T as std::ops::Sub<T>>::Output: Into<<T as std::ops::Add<T>>::Output>,
    <T as std::ops::Add>::Output: std::cmp::Ord,
{
    #[inline]
    pub fn expand(self, rhs: T) -> Interval<<T as std::ops::Add<T>>::Output> {
        Interval::new(self.from + rhs.clone(), (self.to - rhs).into())
    }
}

impl<T, U> PartialEq<Interval<U>> for Interval<T>
where
    T: PartialEq<U> + Ord,
    U: Ord,
{
    #[inline]
    fn eq(&self, rhs: &Interval<U>) -> bool {
        self.from == rhs.from && self.to == rhs.to
    }
}

impl<T> Clone for Interval<T>
where
    T: Clone + Ord,
{
    #[inline]
    fn clone(&self) -> Self {
        Interval::new(self.from.clone(), self.to.clone())
    }
}

impl<T> Copy for Interval<T> where T: Copy + Ord {}

impl<T> std::fmt::Display for Interval<T>
where
    T: std::fmt::Display + Ord,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{from}->{to}]", from = self.from, to = self.to)
    }
}

impl<T: std::cmp::Ord + Default> Default for Interval<T> {
    fn default() -> Self {
        Self {
            from: T::default(),
            to: T::default(),
        }
    }
}

#[cfg(test)]
#[test]
fn test_contains() {
    assert!(Interval::new(0, 10).contains(&5));
    assert!(Interval::new(0, 10).contains(&0));
    assert!(Interval::new(0, 10).contains(&10));
    assert!(!Interval::new(0, 10).contains(&-1));
    assert!(!Interval::new(0, 10).contains(&11));

    assert!(Interval::new(10, 0).contains(&10));
    assert!(Interval::new(10, 0).contains(&0));
    assert!(Interval::new(10, 0).contains(&11));
    assert!(Interval::new(10, 0).contains(&-1));
    assert!(!Interval::new(10, 0).contains(&5));
}
