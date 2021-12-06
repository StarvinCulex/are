// by *StarvinCulex @2021/11/27*

/// 计算区间[`interval`]包含范围0..[`size`]中多少个元素
#[inline]
pub fn measure_length(size: isize, interval: Interval<isize>) -> isize {
    if interval.to >= interval.from {
        interval.to - interval.from + 1
    } else {
        size + interval.to - interval.from + 1
    }
}

/// 计算二维区间[`area`]包含大小是[`matrix_size`]的矩阵中多少个元素
/// ```rust
/// matrix_size.reduce(area, measure_length)
#[inline]
pub fn measure_area(matrix_size: Coord<isize>, area: Coord<Interval<isize>>) -> Coord<isize> {
    matrix_size.reduce(area, measure_length)
}

#[cfg(test)]
#[test]
fn test_measure_length() {
    assert_eq!(measure_length(10, Interval::new(5, 7)), 3);
    assert_eq!(measure_length(10, Interval::new(7, 5)), 9);
}
