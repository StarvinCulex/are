use crate::Matrix;

pub trait ConEncoder {
    fn flush(&mut self, grids: &'_ Matrix<char, 1, 1>) -> Vec<char>;

    fn update(&mut self, grids: &'_ Matrix<char, 1, 1>) -> Vec<char>;

    fn bell(&mut self) -> Vec<char>;
}
