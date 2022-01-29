use crate::arena::Cosmos;
use crate::{measure_area, Coord, Interval, Matrix};

use super::color::Style;

pub struct Screen {
    pub encoder: Box<dyn crate::conencode::ConEncoder>,
    pub display_area: Coord<Interval<isize>>,
}

impl Screen {
    pub fn new(
        display_area: Coord<Interval<isize>>,
        encoder: Box<dyn crate::conencode::ConEncoder>,
    ) -> Self {
        Screen {
            encoder,
            display_area,
        }
    }

    pub fn render(&mut self, cosmos: &Cosmos) -> Vec<u8> {
        todo!()
    }

    pub fn render_map(&mut self, cosmos: &Cosmos) -> Matrix<char, 1, 1> {
        const GRID_WIDTH: usize = 3;
        const ROW_SPLITTER: [char; 1] = [' '];
        let display_area_size: Coord<usize> = measure_area(*cosmos.plate.size(), self.display_area)
            .try_into()
            .unwrap();
        let display_grids = Matrix::<char, 1, 1>::new(
            &(display_area_size * Coord(GRID_WIDTH + ROW_SPLITTER.len(), 1)),
        );

        todo!()
    }
}
