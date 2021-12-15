use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParseGridError {
    NoData,
    InvalidData,
    InconsistentWidth,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cell {
    value: usize,
}

struct Grid {
    cells: Vec<Cell>,
    rows: usize,
    cols: usize,
}

impl<'a> Grid {
    fn get(&'a self, x: usize, y: usize) -> &'a Cell {
        &self.cells[y * self.cols + x]
    }

    fn basin_size_at(&self, x: usize, y: usize) -> usize {
        let mut visited = HashSet::new();
        let mut pending = vec![(x, y)];
        while !pending.is_empty() {
            let (x, y) = pending.swap_remove(0);
            if visited.contains(&(x, y)) || self.get(x, y).value == 9 {
                continue;
            }
            visited.insert((x, y));
            if x > 0 {
                pending.push((x - 1, y));
            }
            if x + 1 < self.cols {
                pending.push((x + 1, y));
            }
            if y > 0 {
                pending.push((x, y - 1));
            }
            if y + 1 < self.rows {
                pending.push((x, y + 1));
            }
        }
        visited.len()
    }

    fn lowest(&self) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        for y in 0..self.rows {
            for x in 0..self.cols {
                let cell = self.get(x, y);
                if x > 0 && cell.value >= self.get(x - 1, y).value {
                    continue;
                }
                if x + 1 < self.cols && cell.value >= self.get(x + 1, y).value {
                    continue;
                }
                if y > 0 && cell.value >= self.get(x, y - 1).value {
                    continue;
                }
                if y + 1 < self.rows && cell.value >= self.get(x, y + 1).value {
                    continue;
                }
                result.push((x, y));
            }
        }
        result
    }

    fn from_lines<S: AsRef<str>>(
        mut lines: impl Iterator<Item = S>,
    ) -> Result<Grid, ParseGridError> {
        let first = lines.next().ok_or(ParseGridError::NoData)?;
        let mut cells = row_to_cells(first.as_ref())?;
        let cols = cells.len();
        let mut rows = 1;
        for line in lines {
            let mut row = row_to_cells(line.as_ref())?;
            if row.len() != cols {
                return Err(ParseGridError::InconsistentWidth);
            }
            cells.append(&mut row);
            rows += 1;
        }
        Ok(Grid { cells, rows, cols })
    }
}

fn row_to_cells(s: &str) -> Result<Vec<Cell>, ParseGridError> {
    s.chars()
        .map(|c| {
            c.to_digit(10)
                .map(|d| Cell { value: d as usize })
                .ok_or(ParseGridError::InvalidData)
        })
        .collect()
}

fn main() {
    let rows = app::read_lines(&app::input_arg());
    let grid = Grid::from_lines(rows).unwrap();

    let low_points = grid.lowest();
    let mut basin_sizes: Vec<usize> = low_points
        .iter()
        .map(|&(x, y)| grid.basin_size_at(x, y))
        .collect();
    basin_sizes.sort();
    let product: usize = basin_sizes.iter().rev().take(3).product();
    println!("The product of the three largest basins is {}", product);
}
