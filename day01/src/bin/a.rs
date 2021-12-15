fn main() {
    let xs = app::read_uints(&app::input_arg());
    // Assumption: All values are positive, non-zero.
    let (_, result) = xs.fold((0, 0), |(prev, acc), x| {
        if prev != 0 && prev < x {
            (x, acc + 1)
        } else {
            (x, acc)
        }
    });

    println!("The number of depth increases is {:?}", result);
}
