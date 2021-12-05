// by *StarvinCulex @2021/11/27*

#[inline]
pub fn measure_length(size: isize, interval: Interval<isize>) -> isize {
    if interval.to >= interval.from {
        interval.to - interval.from + 1
    } else {
        size + interval.to - interval.from + 1
    }
}

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
