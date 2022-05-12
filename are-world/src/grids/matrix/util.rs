use std::ops::{Add, Sub};

/// by *StarvinCulex @2021/11/27*
use super::{Coord, Interval};

/// 计算区间[`interval`]包含范围0..[`size`]中多少个元素
#[inline]
pub fn measure_length<I>(size: I, interval: Interval<I>) -> I
where
    I: Ord + Sub<Output = I> + Add<Output = I> + From<u8>,
{
    if interval.to >= interval.from {
        interval.to - interval.from + 1u8.into()
    } else {
        size + interval.to - interval.from + 1u8.into()
    }
}

/// 计算二维区间[`area`]包含大小是[`matrix_size`]的矩阵中多少个元素
/// ```rust
/// matrix_size.reduce(area, measure_length)
#[inline]
pub fn measure_area<I>(matrix_size: Coord<I>, area: Coord<Interval<I>>) -> Coord<I>
where
    I: Ord + Sub<Output = I> + Add<Output = I> + From<u8>,
{
    matrix_size.reduce(area, measure_length::<I>)
}

/// 计算int1和int2在长度是size的环形空间的距离
/// 如果int1和int2重叠（不包括端点重叠），则结果**未定义**
#[inline]
pub fn measure_distance(size: isize, int1: Interval<isize>, int2: Interval<isize>) -> isize {
    let p = (int1.to - int2.from).abs();
    let q = (int1.from - int2.to).abs();
    if p > q {
        q.min(size - p)
    } else {
        p.min(size - q)
    }
}

#[inline]
pub fn measure_distances(
    size: Coord<isize>,
    int1: Coord<Interval<isize>>,
    int2: Coord<Interval<isize>>,
) -> Coord<isize> {
    Coord(
        measure_distance(size.0, int1.0, int2.0),
        measure_distance(size.1, int1.1, int2.1),
    )
}

#[cfg(test)]
#[test]
fn test_measure_length() {
    assert_eq!(measure_length(10, Interval::new(5, 7)), 3);
    assert_eq!(measure_length(10, Interval::new(7, 5)), 9);
}

#[cfg(test)]
#[test]
fn test_measure_distance() {
    assert_eq!(
        measure_distance(1000, Interval::new(1, 2), Interval::new(10, 20)),
        8
    );
    assert_eq!(
        measure_distance(1000, Interval::new(110, 120), Interval::new(130, 10)),
        10
    );
    assert_eq!(
        measure_distance(1000, Interval::new(110, 120), Interval::new(800, 900)),
        210
    );
    assert_eq!(
        measure_distance(1000, Interval::new(150, 250), Interval::new(950, 50)),
        100
    );
}
