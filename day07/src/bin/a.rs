use app;

fn dist(a: usize, b: usize) -> usize {
    if a >= b {
        a - b
    } else {
        b - a
    }
}

fn cost<'a>(positions: impl Iterator<Item = &'a usize>, target: usize) -> usize {
    positions.fold(0, |acc, &x| acc + dist(x, target))
}

fn main() {
    let line = app::read_line(&app::input_arg());
    let positions: Vec<usize> = line
        .split(',')
        .map(|n| n.parse::<usize>().expect("Invalid input"))
        .collect();

    let min = *positions.iter().min().expect("Input is empty");
    let max = *positions.iter().max().expect("Input is empty");

    let mut least_cost = usize::MAX;
    let mut least_pos = 0;
    for i in min..=max {
        let cur_cost = cost(positions.iter(), i);
        if cur_cost < least_cost {
            least_cost = cur_cost;
            least_pos = i;
        }
    }
    println!(
        "The minimum cost is {} at position {}",
        least_cost, least_pos
    );
}
