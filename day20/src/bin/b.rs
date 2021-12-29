use app::grid::Grid;

const NUM_ITERS: usize = 50;

fn str_to_bools(s: &str) -> Vec<bool> {
    s.chars()
        .map(|c| match c {
            '.' => false,
            '#' => true,
            _ => panic!("Invalid char {}", c),
        })
        .collect()
}

fn read_grid(iter: &mut dyn Iterator<Item = Vec<bool>>) -> Grid<bool> {
    let mut cells: Vec<bool> = Vec::new();
    let lines: Vec<_> = iter.collect();
    if lines.is_empty() {
        panic!("Missing input grid");
    }
    let inner_rows = lines.len();
    let inner_cols = lines[0].len();
    // Because we're using a 3x3 kernel, every iteration can grow the grid by 2 in each direction.
    // We need room for its growth, plus padding to avoid branching in the kernel.
    let padding = 2 * (NUM_ITERS + 1);
    let rows = inner_rows + 2 * padding;
    let cols = inner_cols + 2 * padding;
    cells.resize(rows * cols, false);
    let mut offset = padding * cols + padding; // Skip padding rows and columns
    for line in lines {
        if line.len() != inner_cols {
            panic!("Inconsistent line width");
        }
        cells[offset..(offset + inner_cols)].clone_from_slice(&line[..]);
        offset += cols;
    }
    Grid::new_with_cells(cells, rows, cols)
}

fn iterate(grid: &Grid<bool>, rules: &[bool], inset: usize) -> Grid<bool> {
    let (rows, cols) = grid.size();
    let mut result = Grid::new(rows, cols);
    for row in inset..(rows - inset) {
        for col in inset..(cols - inset) {
            let index = grid[row - 1][col - 1..=col + 1]
                .iter()
                .chain(grid[row][col - 1..=col + 1].iter())
                .chain(grid[row + 1][col - 1..=col + 1].iter())
                .fold(0, |n, &b| (n << 1) | if b { 1 } else { 0 });
            result[row][col] = rules[index];
        }
    }

    if rules[0] {
        // Toggle all the border pixels
        let val = !grid[0][0];
        for row_num in 0..rows {
            let row = &mut result[row_num];
            if row_num < inset || row_num >= rows - inset {
                // Top/bottom border
                row.fill(val);
            } else {
                // Left/right border
                row[..inset].fill(val);
                row[cols - inset..].fill(val);
            }
        }
    }

    result
}

fn main() {
    let mut lines = app::read_lines(&app::input_arg()).map(|s| str_to_bools(&s));

    let rules = lines.next().expect("Missing rule set");
    if rules[0] && rules[511] {
        panic!("Cannot solve when 3x3 dark becomes light (solution is infinite)");
    }
    lines.next();

    let mut grid = read_grid(&mut lines);
    let mut inset = 2 * (NUM_ITERS + 1);
    for _ in 0..NUM_ITERS {
        inset -= 2;
        grid = iterate(&grid, &rules, inset);
    }

    let lit = grid.iter().filter(|(_, _, &b)| b).count();
    println!("There are {} cells lit", lit);
}
