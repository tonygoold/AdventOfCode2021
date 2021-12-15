use app::grid::Grid;
use app::point::Point2D;

const REPEAT_FACTOR: usize = 5;

type Point = Point2D<usize>;

#[derive(Debug, Clone, Copy)]
struct Trail {
    cur: Point,
    prev: Point,
    cost: usize,
}

impl Trail {
    fn new(cur: Point, prev: Point, cost: usize) -> Self {
        Trail { cur, prev, cost }
    }
}

fn repeat_grid(g: &Grid<usize>, factor: usize) -> Grid<usize> {
    let (init_rows, init_cols) = g.size();
    let (dest_rows, dest_cols) = (init_rows * factor, init_cols * factor);
    let mut result = Grid::new(dest_rows, dest_cols);
    g.enumerate(|(x, y), val| {
        for fy in 0..factor {
            for fx in 0..factor {
                let cost = (val + fx + fy - 1) % 9 + 1;
                result[fy * init_rows + y][fx * init_cols + x] = cost;
            }
        }
    });
    result
}

// Each cell indicates the closest previous
fn shortest_paths(g: &Grid<usize>, from: (usize, usize)) -> Grid<usize> {
    let (rows, cols) = g.size();
    let mut unvisited: Vec<Trail> = g
        .iter()
        .map(|(x, y, _)| {
            Trail::new(
                Point::new(x, y),
                Point::new(x, y),
                if x == from.0 && y == from.1 {
                    0
                } else {
                    usize::MAX
                },
            )
        })
        .collect();
    unvisited.sort_unstable_by(|a, b| b.cost.cmp(&a.cost));
    let mut visited = Vec::new();
    while let Some(p) = unvisited.pop() {
        let base_cost = p.cost;
        if base_cost == usize::MAX {
            panic!("Least element in set is usize::MAX");
        }

        let candidates = unvisited.iter_mut().filter(|trail| {
            let cur = &trail.cur;
            cur.x == p.cur.x && (cur.y == p.cur.y + 1 || p.cur.y == cur.y + 1)
                || cur.y == p.cur.y && (cur.x == p.cur.x + 1 || p.cur.x == cur.x + 1)
        });
        for c in candidates {
            let cost = base_cost + g[c.cur.y][c.cur.x];
            let prev_best = c.cost;
            if cost < prev_best {
                c.prev = p.cur;
                c.cost = cost;
            }
        }
        unvisited.sort_unstable_by(|a, b| b.cost.cmp(&a.cost));

        visited.push(p);
    }
    let mut result = Grid::new(rows, cols);
    visited.into_iter().for_each(|trail| {
        result[trail.cur.y][trail.cur.x] = trail.cost;
    });
    result
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

    let g: Grid<usize> = Grid::new_with_cells(cells, rows, cols);
    let g2 = repeat_grid(&g, REPEAT_FACTOR);
    let shortest = shortest_paths(&g2, (0, 0));
    let shortest_cost = shortest[REPEAT_FACTOR * rows - 1][REPEAT_FACTOR * cols - 1];
    println!("The shortest cost to the bottom right is {}", shortest_cost);
}
