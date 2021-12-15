use app::grid::Grid;

fn grid_incr(grid: &Grid<usize>) -> Grid<usize> {
    grid.map(|_, &val| val + 1)
}

fn count_flashed(grid: &Grid<usize>) -> usize {
    let mut count = 0;
    grid.enumerate(|_, &val| {
        if val == 0 {
            count += 1;
        }
    });
    count
}

fn flash_coord(grid: &mut Grid<usize>, row: usize, col: usize) {
    let (rows, cols) = grid.size();
    let miny = if row > 0 { row - 1 } else { row };
    let maxy = if row + 1 < rows { row + 1 } else { row };
    let minx = if col > 0 { col - 1 } else { col };
    let maxx = if col + 1 < cols { col + 1 } else { col };
    for y in miny..=maxy {
        for x in minx..=maxx {
            if (y != row || x != col) && grid[y][x] > 0 {
                grid[y][x] += 1;
            }
        }
    }
}

fn grid_flash(grid: &Grid<usize>) -> Grid<usize> {
    let (rows, cols) = grid.size();
    let mut g = grid.clone();
    loop {
        let mut stable = true;

        for y in 0..rows {
            for x in 0..cols {
                if g[y][x] > 9 {
                    g[y][x] = 0;
                    flash_coord(&mut g, y, x);
                    stable = false;
                }
            }
        }
        if stable {
            break g;
        }
    }
}

fn main() {
    let mut lines = app::read_lines(&app::input_arg()).map(|s| {
        s.chars()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|c| c.to_digit(10).expect("Invalid digit") as usize)
    });
    let mut cells: Vec<usize> = lines.next().expect("No input").collect();
    let cols = cells.len();
    let mut rows = 1;
    for line in lines {
        let prev_size = cells.len();
        cells.extend(line);
        if cells.len() != prev_size + cols {
            panic!("Inconsistent row width");
        }
        rows += 1;
    }

    let mut g: Grid<usize> = Grid::new_with_cells(cells, rows, cols);
    let mut steps = 0;
    loop {
        steps += 1;
        g = grid_flash(&grid_incr(&g));
        if count_flashed(&g) == rows * cols {
            break;
        }
    }
    println!("Reached a full flash after {} iterations", steps);
}
