use crate::arena::Matrix;
use crate::conencode::conencoder::ConEncoder;
use crate::Coord;

pub struct NVTer {
    last_graph: Matrix<char, 1, 1>,
}

impl NVTer {
    pub fn new() -> Self {
        Self {
            last_graph: Matrix::new(&Coord(0, 0)),
        }
    }
}

impl ConEncoder for NVTer {
    fn flush(&mut self, grids: &'_ Matrix<char, 1, 1>) -> Vec<char> {
        self.last_graph = grids.clone();
        Self::repaint(grids)
    }

    fn update(&mut self, grids: &'_ Matrix<char, 1, 1>) -> Vec<char> {
        self.flush(grids)
    }

    fn bell(&mut self) -> Vec<char> {
        Self::BEL.into()
    }
}

impl NVTer {
    const BEL: [char; 1] = [7 as char];
    const BS: [char; 1] = [8 as char];
    const NEW_LINE: [char; 2] = [13 as char, 10 as char];
    const NEW_PAGE: [char; 3] = [13 as char, 10 as char, 12 as char];

    fn repaint(grids: &Matrix<char, 1, 1>) -> Vec<char> {
        let mut string = Vec::with_capacity(
            (grids.size().0 as usize + Self::NEW_LINE.len()) * grids.size().1 as usize
                + Self::NEW_PAGE.len(),
        );
        string.extend(Self::NEW_PAGE);
        for j in 0..grids.size().1 {
            for i in 0..grids.size().0 {
                string.push(grids[Coord(i, j)]);
            }
            string.extend(Self::NEW_LINE);
        }
        string
    }
}
