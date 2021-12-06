use app;

const NUM_DAYS: usize = 80;

fn elapse_day(fish: &[usize; 9]) -> [usize; 9] {
    let mut result = [0; 9];
    for i in 1..9 {
        result[i - 1] = fish[i];
    }
    result[6] += fish[0];
    result[8] = fish[0];
    result
}

fn main() {
    let line = app::read_line(&app::input_arg());
    let inputs = line
        .split(',')
        .map(|n| n.parse::<usize>().expect("Invalid input"));

    let mut fish = [0usize; 9];
    for input in inputs {
        fish[input] += 1;
    }
    for _ in 0..NUM_DAYS {
        fish = elapse_day(&fish);
    }

    let count: usize = fish.into_iter().sum();
    println!("After {} days there are {} fish", NUM_DAYS, count);
}
