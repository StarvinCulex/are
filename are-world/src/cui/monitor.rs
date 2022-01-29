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
    _p: std::marker::PhantomData<MonitoredGridElement>,
}

impl<E, Rsc, const EDW: usize, const EDH: usize> Monitor<E, Rsc, EDW, EDH>
where
    Rsc: Fn(&E, usize) -> [[char; EDH]; EDW],
{
    pub fn new(grid_count: Coord<usize>, resources: Rsc) -> Self {
        todo!()
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
        todo!()
    }
}
