use array2d::Array2D;
use super::cell::Cell;

#[derive(Debug, Clone)]
pub struct Board{
	grid: Array2D<Cell>
}

impl Board {
    fn new(grid: Array2D<Cell>) -> Self { Self { grid } }
}

impl Default for Board {
    fn default() -> Self {
        Self::new(Array2D::filled_with(Cell{number:0, marked:false}, 5, 5))
    }
}
