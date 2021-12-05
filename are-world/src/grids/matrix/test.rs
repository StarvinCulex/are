//! by *StarvinCulex @2021/11/28*

use super::{Coord, Matrix};

#[cfg(test)]
fn test_sub<const CW: usize, const CH: usize>() {
    let ctor = |opt_pos: Option<Coord<isize>>| {
        if let Some(pos) = opt_pos {
            pos.to_string()
        } else {
            "".to_string()
        }
    };
    let size = Coord(5, 5);
    let mut matrix = Matrix::<String, CW, CH>::with_ctor(&size, ctor);

    assert_eq!(*matrix.size(), Coord(5isize, 5isize));

    for j in 0..5 {
        for i in 0..5 {
            let expected = ctor(Some(Coord(i, j)));
            let value = &matrix[Coord(i, j)];
            assert_eq!(&expected, value);
        }
    }

    for (pos, value) in matrix.iter() {
        assert!(Coord(0, 0) <= pos && pos < matrix.size);
        assert_eq!(*value, ctor(Some(pos)));
    }
}

#[cfg(test)]
#[test]
fn test() {
    test_sub::<1, 1>();
    // test_sub::<2, 2>();
}
