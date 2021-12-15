use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Grid<T> {
    cells: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Grid<T> {
    pub fn new_with_cells(cells: Vec<T>, rows: usize, cols: usize) -> Self {
        Grid { cells, rows, cols }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn enumerate<F>(&self, mut f: F)
    where
        F: FnMut((usize, usize), &T),
    {
        for y in 0..self.rows {
            let row = &self[y];
            for (x, cell) in row.iter().enumerate().take(self.cols) {
                f((x, y), cell);
            }
        }
    }

    pub fn map<F>(&self, mut f: F) -> Self
    where
        F: FnMut((usize, usize), &T) -> T,
    {
        let mut cells = Vec::with_capacity(self.cells.len());
        for y in 0..self.rows {
            let row = &self[y];
            for (x, cell) in row.iter().enumerate().take(self.cols) {
                cells.push(f((x, y), cell));
            }
        }
        Self::new_with_cells(cells, self.rows, self.cols)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.cols;
        let end = start + self.cols;
        &self.cells[start..end]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * self.cols;
        let end = start + self.cols;
        &mut self.cells[start..end]
    }
}

impl<T: Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Grid<T> {
        let mut cells = Vec::new();
        cells.resize_with(rows * cols, T::default);
        Grid { cells, rows, cols }
    }
}

pub struct Iter<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let (rows, cols) = self.grid.size();
        let (x, y) = (self.x, self.y);
        if y == rows {
            return None;
        }
        let item = &self.grid[y][x];
        if x + 1 < cols {
            self.x += 1;
        } else {
            self.x = 0;
            self.y += 1;
        }
        Some((x, y, item))
    }
}
