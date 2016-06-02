mod iter;
mod coord;
mod test;

use traits::Grid;
use traits::Cell;
use traits::Nhood;
use traits::Coord;
use repr::CellRepr;
use repr::GridRepr;

use self::iter::Iter;
pub use self::coord::GridCoord;


pub struct SquareGrid<'a, C, N>
    where C: Cell,
          N: Nhood<Coord=GridCoord> {

    cells: Vec<C>,
    old_cells: Vec<C>,
    nhood: N,
    neighbors: Vec<Vec<Option<usize>>>,
    repr: GridRepr<'a, GridCoord>,
    rows: i32,
    cols: i32,
}


impl<'a, C, N> SquareGrid<'a, C, N>
    where C: Cell,
          N: Nhood<Coord=GridCoord> {

    pub fn new(rows: i32, cols: i32, nhood: N) -> Self {

        let len = (rows * cols) as usize;

        let cells = vec![C::default(); len];
        let old_cells = Vec::with_capacity(len);
        let neighbors = Vec::with_capacity(len);
        let repr = GridRepr::new(rows, cols, None);

        let mut grid = SquareGrid {
            cells: cells,
            old_cells: old_cells,
            nhood: nhood,
            neighbors: neighbors,
            repr: repr,
            rows: rows,
            cols: cols,
        };

        grid.init_neighbors();
        grid.init_cell_reprs();

        grid
    }

    fn init_cell_reprs(&mut self) {

        for (offset, cell) in self.cells.iter().enumerate() {
            let coord = GridCoord::from_offset(offset as i32, self.rows, self.cols);
            let mut repr = CellRepr::new(coord, None);

            cell.repr(&mut repr.state);
            self.repr.cells.push(repr);
        }
    }

    fn init_neighbors(&mut self) {

        let len = self.cells.len() as i32;

        for offset in 0 .. len {
            let coord = GridCoord::from_offset(offset, self.rows, self.cols);

            let neighbors_count = self.nhood.neighbors_count();
            let mut neighbors = Vec::with_capacity(neighbors_count);

            let neighbors_coords = self.nhood.neighbors(coord);
            for coord in neighbors_coords.iter() {

                if coord.x() >= 0 && coord.x() < self.cols &&
                   coord.y() >= 0 && coord.y() < self.rows {

                    neighbors.push(Some(coord.offset(self.cols)));
                }
                else {
                    neighbors.push(None);
                }
            }

            self.neighbors.push(neighbors);
        }
    }

    fn neighbors_iter<'b>(&self,
                          cells: &'b Vec<C>,
                          neighbors: &'b Vec<Option<usize>>) -> Iter<'b, C> {

        Iter::new(cells, neighbors, self.nhood.neighbors_count())
    }
}


impl<'a, C, N> Grid for SquareGrid<'a, C, N>
    where C: Cell,
          N: Nhood<Coord=GridCoord> {

    type Coord = GridCoord;

    fn step(&mut self) {
        self.old_cells = self.cells.clone();

        for (cell_no, cell) in self.old_cells.iter().enumerate() {
            let ref neighbors = self.neighbors[cell_no];
            let neighbors_iter = self.neighbors_iter(&self.old_cells, &neighbors);

            let new_cell = cell.step(neighbors_iter);

            // update representation
            let ref mut cell_repr = self.repr.cells[cell_no];
            new_cell.repr(&mut cell_repr.state);

            self.cells[cell_no] = new_cell;
        }
    }

    fn repr(&self) -> &GridRepr<GridCoord> {
        &self.repr
    }

    fn from_repr<'b: 'c, 'c, Crd: Coord>(&'b mut self, repr: &GridRepr<'c, Crd>) {

        debug_assert_eq!(self.rows, repr.rows);
        debug_assert_eq!(self.cols, repr.cols);

        for cell_repr in repr.cells.iter() {
            let c = GridCoord::from_2d(cell_repr.coord.x(), cell_repr.coord.y());
            let offset = c.offset(self.cols);

            let ref mut cell = self.cells[offset];
            cell.from_repr(&cell_repr.state);

            let ref mut inner_repr = self.repr.cells[offset];
            cell.repr(&mut inner_repr.state);
        }
    }
}
