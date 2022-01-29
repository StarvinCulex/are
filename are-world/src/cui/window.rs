//! by *StarvinCulex* @ 2022/01/16

use crate::{Coord, Matrix};

pub trait Window {
    fn set_size(&mut self, size: Coord<usize>) -> Result<(), ()>;

    fn render(&mut self) -> Matrix<char, 1, 1>;
}
