#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    row: usize,
    col: usize,
    offset_index: usize,
}

impl<'a, T> Iterator for GridIterator<'a, T> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let offsets: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        while self.offset_index < offsets.len() {
            let (row_offset, col_offset) = offsets[self.offset_index];
            self.offset_index += 1;

            let neighbour_row = self.row as isize + row_offset;
            let neighbour_col = self.col as isize + col_offset;

            if neighbour_row >= 0
                && neighbour_row < self.grid.rows as isize
                && neighbour_col >= 0
                && neighbour_col < self.grid.cols as isize
            {
                return Some((neighbour_row as usize, neighbour_col as usize));
            }
        }

        None
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        let size = rows * cols;
        Grid {
            rows,
            cols,
            grid: vec![T::default(); size],
        }
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        assert_eq!(grid.len(), rows * cols);
        Grid {
            rows,
            cols,
            grid: grid.to_vec(),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        let index = row * self.cols + col;
        &self.grid[index]
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        let index = row * self.cols + col;
        self.grid[index] = value;
    }

    pub fn neighbours(&self, row: usize, col: usize) -> GridIterator<T> {
        GridIterator {
            grid: self,
            row,
            col,
            offset_index: 0,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Dead
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq)]
pub struct GameOfLife {
    grid: Grid<Cell>,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        GameOfLife { grid }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        &self.grid
    }

    pub fn step(&mut self) {
        let (rows, cols) = self.grid.size();
        let total_cells = rows * cols;

        let mut next_states: Vec<Cell> = Vec::with_capacity(total_cells);

        for row in 0..rows {
            for col in 0..cols {
                let cell_state = *self.grid.get(row, col);

                let live_neighbor_count = self
                    .grid
                    .neighbours(row, col)
                    .filter(|&(r, c)| *self.grid.get(r, c) == Cell::Alive)
                    .count();

                let next_state = match (cell_state, live_neighbor_count) {
                    (Cell::Alive, 2) | (Cell::Alive, 3) | (Cell::Dead, 3) => Cell::Alive,
                    _ => Cell::Dead,
                };

                next_states.push(next_state);
            }
        }

        for (index, &next_state) in next_states.iter().enumerate() {
            self.grid.grid[index] = next_state;
        }
    }
}
