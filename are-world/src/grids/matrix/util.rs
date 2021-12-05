// by *StarvinCulex @2021/11/27*

#[inline]
pub fn measure_length(size: isize, interval: Interval<isize>) -> isize {
    if interval.to >= interval.from {
        interval.to - interval.from + 1
    } else {
        size - (interval.to - interval.from - 1)
    }
}

#[inline]
pub fn measure_area(matrix_size: Coord<isize>, area: Coord<Interval<isize>>) -> Coord<isize> {
    Coord(
        measure_length(matrix_size.0, area.0),
        measure_length(matrix_size.1, area.1),
    )
}
