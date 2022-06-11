//! by *StarvinCulex @2021/10/24*

use crate::o;
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
use std::fmt::Formatter;
use std::intrinsics::likely;

#[derive(Hash, Debug, Serialize, Deserialize, Clone, Copy, Eq, Default)]
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
    #[inline]
    pub fn contains(&self, point: &T) -> bool {
        let greater_than_from = point >= &self.from;
        let less_than_to = point <= &self.to;
        let closed_interval = self.from <= self.to;

        closed_interval ^ greater_than_from ^ less_than_to
        // if closed_interval {
        //     greater_than_from && less_than_to
        // } else {
        //     greater_than_from || less_than_to
        // }
    }

    /// 判断`interval`是否属于`self`表示的范围
    /// 要求 self 和 interval 均在范围内
    #[inline]
    pub fn contains_interval(&self, interval: &Self, size: T) -> bool
    where
        T: std::ops::Add<T> + From<bool> + Copy,
        <T as std::ops::Add<T>>::Output: Eq,
    {
        let self_closed_interval = self.from <= self.to;
        let other_closed_interval = interval.from <= interval.to;

        if likely(self_closed_interval) {
            other_closed_interval && self.from <= interval.from && interval.to <= self.to
                || self.from == false.into() && self.to.add(true.into()) == size.add(false.into())
        } else if likely(other_closed_interval) {
            interval.to <= self.to
                || self.from <= interval.from
                || self.to.add(true.into()) == self.from.add(false.into())
        } else {
            interval.from >= self.from && self.to >= interval.to
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
    <T as std::ops::Add>::Output: Ord,
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

impl<T> std::fmt::Display for Interval<T>
where
    T: std::fmt::Display + Ord,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{from}->{to}]", from = self.from, to = self.to)
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

    // closed closed
    assert_eq!(
        Interval::new(114, 514).contains_interval(&Interval::new(114, 514), 1000),
        true
    );
    assert_eq!(
        Interval::new(114, 514).contains_interval(&Interval::new(115, 514), 1000),
        true
    );
    assert_eq!(
        Interval::new(114, 514).contains_interval(&Interval::new(114, 513), 1000),
        true
    );
    assert_eq!(
        Interval::new(114, 514).contains_interval(&Interval::new(114, 515), 1000),
        false
    );
    assert_eq!(
        Interval::new(114, 514).contains_interval(&Interval::new(113, 514), 1000),
        false
    );

    // closed opened
    assert_eq!(
        Interval::new(114, 514).contains_interval(&Interval::new(116, 115), 1000),
        false
    );
    assert_eq!(
        Interval::new(114, 514).contains_interval(&Interval::new(999, 0), 1000),
        false
    );
    assert_eq!(
        Interval::new(0, 999).contains_interval(&Interval::new(514, 114), 1000),
        true
    );

    // opened closed
    assert_eq!(
        Interval::new(514, 114).contains_interval(&Interval::new(0, 114), 1000),
        true
    );
    assert_eq!(
        Interval::new(514, 114).contains_interval(&Interval::new(514, 999), 1000),
        true
    );
    assert_eq!(
        Interval::new(514, 114).contains_interval(&Interval::new(115, 115), 1000),
        false
    );
    assert_eq!(
        Interval::new(500, 499).contains_interval(&Interval::new(114, 514), 1000),
        true
    );

    // opened opened
    assert_eq!(
        Interval::new(514, 114).contains_interval(&Interval::new(514, 114), 1000),
        true
    );
    assert_eq!(
        Interval::new(514, 114).contains_interval(&Interval::new(515, 114), 1000),
        true
    );
    assert_eq!(
        Interval::new(514, 114).contains_interval(&Interval::new(514, 113), 1000),
        true
    );
    assert_eq!(
        Interval::new(514, 114).contains_interval(&Interval::new(514, 115), 1000),
        false
    );
    assert_eq!(
        Interval::new(514, 114).contains_interval(&Interval::new(513, 114), 1000),
        false
    );
}
#[cfg(test)]
#[test]
fn test_contains_interval() {
    assert!(o!(0=>10).contains_interval(&o!(0=>10), 100));
    assert!(!o!(110=>120).contains_interval(&o!(130=>10), 1000));
    assert!(!o!(110=>120).contains_interval(&o!(130=>10), 1000));
    assert!(!o!(110=>120).contains_interval(&o!(10=>109), 1000));
    assert!(o!(110=>120).contains_interval(&o!(110=>111), 1000));
    assert!(o!(900=>100).contains_interval(&o!(0=>100), 1000));
    assert!(o!(900=>100).contains_interval(&o!(0=>10), 1000));
    assert!(o!(900=>100).contains_interval(&o!(900=>11100), 1000));
    assert!(o!(900=>100).contains_interval(&o!(910=>90), 1000));
    assert!(!o!(900=>100).contains_interval(&o!(890=>910), 1000));
    assert!(!o!(900=>100).contains_interval(&o!(890=>10), 1000));
    assert!(!o!(900=>100).contains_interval(&o!(0=>110), 1000));
    assert!(!o!(900=>100).contains_interval(&o!(120=>130), 1000));
    assert!(!o!(900=>100).contains_interval(&o!(120=>10), 1000));
    assert!(!o!(0=>1000).contains_interval(&o!(900=>100), 1000));
}
