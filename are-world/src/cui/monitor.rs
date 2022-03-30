//! by *StarvinCulex* @2022/01.29

use crate::cui::window::Window;
use crate::{Coord, Matrix};

pub struct Monitor<
    MonitoredGridElement,
    Resource: Fn(&MonitoredGridElement, usize) -> [[char; ELEMENT_DISPLAY_HEIGHT]; ELEMENT_DISPLAY_WIDTH],
    const ELEMENT_DISPLAY_WIDTH: usize,
    const ELEMENT_DISPLAY_HEIGHT: usize,
> {
    pub resource: Resource,
    pub window: Option<Box<Matrix<MonitoredGridElement, 1, 1>>>,
    _p: std::marker::PhantomData<MonitoredGridElement>,
    phase: usize,
}

impl<E, Rsc, const EDW: usize, const EDH: usize> Monitor<E, Rsc, EDW, EDH>
where
    Rsc: Fn(&E, usize) -> [[char; EDH]; EDW],
{
    pub fn new(grid_size: Coord<usize>, resource: Rsc) -> Self {
        Monitor {
            resource,
            window: None,
            phase: 0,
            _p: std::marker::PhantomData::default(),
        }
    }

    pub fn put(&mut self, matrix: Matrix<E, 1, 1>) {
        self.window = Some(Box::new(matrix));
    }

    pub fn clear(&mut self) {
        self.window = None;
    }
}

impl<E, Rsc, const EDW: usize, const EDH: usize> Window for Monitor<E, Rsc, EDW, EDH>
where
    Rsc: Fn(&E, usize) -> [[char; EDH]; EDW],
{
    fn set_size(&mut self, size: Coord<usize>) -> Result<(), ()> {
        todo!()
    }

    fn render(&mut self) -> Matrix<char, 1, 1> {
        let m = match &self.window {
            None => Matrix::new(Coord(0usize, 0usize)),
            Some(w) => {
                let texts = w.as_area().map(|e| (self.resource)(e, self.phase));
                Matrix::with_ctor(
                    (*texts.size()).try_into().unwrap_or(Coord(0usize, 0usize)) * Coord(EDW, EDH),
                    |p| {
                        let grid = Coord(p.0 / EDW as isize, p.1 / EDH as isize);
                        let offset = Coord(p.0 % EDW as isize, p.1 % EDH as isize);
                        texts[grid][offset.0 as usize][offset.1 as usize]
                    },
                )
            }
        };
        self.phase += 1;
        m
    }
}
