//! by *StarvinCulex @2021/10/24*
/// 可以表示二维平面中位置信息的数对结构
///
/// # 如何构造
/// ## Example:
/// ```rust
/// Coord(1, 2)
/// ```
///
/// 前面的数应该用 _**x**_ 或 _**i**_ 字母表示
///
/// 后面的数应该用 _**y**_ 或 _**j**_ 字母表示
///
/// > 支持的每种操作可见源码。（其中的所有实现都只用了一行，没必要写说明吧？）
///
/// # 和[`Interval`](super::interval::Interval)混合使用 ##
/// 使用`Coord<Interval<T>>`表示二维的区间。
/// 在[`grids/coord_interval.rs`](super::coord_interval)中，
/// [`Coord`]重载了[`bitor`](std::ops::BitOr)运算符，
/// 用于将两个[`Coord<T>`]组合成一个[`Coord<Interval<T>>`]:
///
/// ## Example:
/// ```rust
/// assert_eq!(
///     Coord(Interval::new(1, 2), Interval::new(3, 4)),
///     Coord(1, 3) | Coord(2, 4)
/// );
/// ```
///
/// 此外， [`grids/coord_interval.rs`](super::coord_interval)还为[`Coord<Interval>`]增加了以下方法
/// * [`from`]
/// * [`to`]
/// * [`contains`]
/// * [`contains_point`]
/// * [`offset`]
use serde::{Deserialize, Serialize};

#[derive(Hash, Debug, Serialize, Deserialize)]
pub struct Coord<T>(pub T, pub T);

#[allow(dead_code)]
impl<T> Coord<T> {
    /// ```rust
    /// return Coord(f(self.0), f(self.1));
    #[inline]
    pub fn map<U>(self, f: impl Fn(T) -> U) -> Coord<U> {
        Coord(f(self.0), f(self.1))
    }
    /// ```rust
    /// return Coord(f(self.0, rhs.0), f(self.1, rhs.1));
    #[inline]
    pub fn reduce<U, V>(self, rhs: Coord<U>, f: impl Fn(T, U) -> V) -> Coord<V> {
        Coord(f(self.0, rhs.0), f(self.1, rhs.1))
    }
    /// ```rust
    /// return f(self.0, self.1);
    #[inline]
    pub fn merge<U>(self, f: impl Fn(T, T) -> U) -> U {
        f(self.0, self.1)
    }
    /// ```rust
    /// return Coord(self.1, self.0);
    #[inline]
    pub fn reverse(self) -> Self {
        Coord(self.1, self.0)
    }
}

impl<T, R> std::ops::Add<Coord<R>> for Coord<T>
where
    T: std::ops::Add<R>,
{
    type Output = Coord<<T as std::ops::Add<R>>::Output>;
    #[inline]
    fn add(self, rhs: Coord<R>) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T, R> std::ops::Sub<Coord<R>> for Coord<T>
where
    T: std::ops::Sub<R>,
{
    type Output = Coord<<T as std::ops::Sub<R>>::Output>;
    fn sub(self, rhs: Coord<R>) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T, R> std::ops::Mul<Coord<R>> for Coord<T>
where
    T: std::ops::Mul<R>,
{
    type Output = Coord<<T as std::ops::Mul<R>>::Output>;
    #[inline]
    fn mul(self, rhs: Coord<R>) -> Self::Output {
        Coord(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl<T, R> std::ops::Div<Coord<R>> for Coord<T>
where
    T: std::ops::Div<R>,
{
    type Output = Coord<<T as std::ops::Div<R>>::Output>;
    #[inline]
    fn div(self, rhs: Coord<R>) -> Self::Output {
        Coord(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl<T, R> std::ops::Rem<Coord<R>> for Coord<T>
where
    T: std::ops::Rem<R>,
{
    type Output = Coord<<T as std::ops::Rem<R>>::Output>;
    #[inline]
    fn rem(self, rhs: Coord<R>) -> Self::Output {
        Coord(self.0 % rhs.0, self.1 % rhs.1)
    }
}

impl<T: std::ops::Neg> std::ops::Neg for Coord<T> {
    type Output = Coord<<T as std::ops::Neg>::Output>;
    #[inline]
    fn neg(self) -> Self::Output {
        Coord(-self.0, -self.1)
    }
}

#[allow(dead_code)]
impl<T: std::ops::Neg<Output = T>> Coord<T> {
    /// ```rust
    /// return Coord(-self.0, self.1);
    #[inline]
    fn neg0(self) -> Self {
        Coord(-self.0, self.1)
    }
    /// ```rust
    /// return Coord(self.0, -self.1);
    #[inline]
    fn neg1(self) -> Self {
        Coord(self.0, -self.1)
    }
}

impl<T: Clone> Clone for Coord<T> {
    fn clone(&self) -> Self {
        Coord(self.0.clone(), self.1.clone())
    }
}

impl<T: Copy> Copy for Coord<T> {}

impl<T, R> PartialEq<Coord<R>> for Coord<T>
where
    T: PartialEq<R>,
{
    #[inline]
    fn eq(&self, rhs: &Coord<R>) -> bool {
        self.0 == rhs.0 && self.1 == rhs.1
    }
}

impl<T: PartialEq + Eq> Eq for Coord<T> {}

impl<T, R> PartialOrd<Coord<R>> for Coord<T>
where
    T: PartialOrd<R>,
{
    fn partial_cmp(&self, rhs: &Coord<R>) -> std::option::Option<std::cmp::Ordering> {
        match self.0.partial_cmp(&rhs.0) {
            None => None,
            Some(std::cmp::Ordering::Less) => {
                if self.1 <= rhs.1 {
                    Some(std::cmp::Ordering::Less)
                } else {
                    None
                }
            }
            Some(std::cmp::Ordering::Equal) => self.1.partial_cmp(&rhs.1),
            Some(std::cmp::Ordering::Greater) => {
                if self.1 >= rhs.1 {
                    Some(std::cmp::Ordering::Greater)
                } else {
                    None
                }
            }
        }
    }
}

pub auto trait NotCoord {}
impl<T> !NotCoord for Coord<T> {}
impl<T: Copy + NotCoord> From<T> for Coord<T> {
    fn from(t: T) -> Self {
        Coord(t, t)
    }
}

impl<T: Default> Default for Coord<T> {
    fn default() -> Self {
        Coord(T::default(), T::default())
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Coord<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({0}, {1})", self.0, self.1)
    }
}
