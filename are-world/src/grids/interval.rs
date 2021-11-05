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
#[derive(Hash, Debug)]
pub struct Interval<T>
where
    T: PartialOrd,
{
    pub from: T,
    pub to: T,
}

#[allow(dead_code)]
impl<T> Interval<T>
where
    T: PartialOrd,
{
    /// ```rust
    /// return Interval{from, to};
    #[inline]
    pub fn new(from: T, to: T) -> Interval<T> {
        Interval { from, to }
    }

    /// 判断`other`表示的范围是否是`self`的子集
    #[inline]
    pub fn contains(&self, other: &Interval<T>) -> bool {
        Interval::raw_contains(&self.from, &self.to, &other.from, &other.to)
    }

    /// 判断`point`是否属于`self`表示的范围
    pub fn contains_point(&self, point: &T) -> bool {
        if self.from <= self.to {
            &self.from <= point && point <= &self.to
        } else {
            point <= &self.from || &self.to <= point
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
    T: PartialOrd + std::ops::Add + Clone,
    <T as std::ops::Add>::Output: PartialOrd,
{
    /// 将`self.from`和`self.to`都加上`rhs`得到的值
    #[inline]
    pub fn offset(self, rhs: T) -> Interval<<T as std::ops::Add>::Output> {
        Interval::new(self.from + rhs.clone(), self.to + rhs)
    }
}

impl<T, U> PartialEq<Interval<U>> for Interval<T>
where
    T: PartialEq<U> + PartialOrd,
    U: PartialOrd,
{
    #[inline]
    fn eq(&self, rhs: &Interval<U>) -> bool {
        self.from == rhs.from && self.to == rhs.to
    }
}

impl<T> Eq for Interval<T> where T: Eq + PartialOrd {}

impl<T> Clone for Interval<T>
where
    T: Clone + PartialOrd,
{
    #[inline]
    fn clone(&self) -> Self {
        Interval::new(self.from.clone(), self.to.clone())
    }
}

impl<T> Copy for Interval<T> where T: Copy + PartialOrd {}

impl<T> std::fmt::Display for Interval<T>
where
    T: std::fmt::Display + PartialOrd,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}->{}]", self.from, self.to)
    }
}

// private
impl<T> Interval<T>
where
    T: PartialOrd,
{
    #[inline]
    fn raw_contains(parent_from: &T, parent_to: &T, child_from: &T, child_to: &T) -> bool {
        if parent_from <= parent_to {
            parent_from <= child_from && child_from <= child_to && child_to <= parent_to
        } else if child_from <= child_to {
            child_to <= parent_to || parent_from <= child_from
        } else {
            parent_from <= child_from && child_to <= parent_to
        }
    }
}
