use app;

fn main() {
    let xs = app::read_uints(&app::input_arg());
    /*
    Assumption: All values are positive, non-zero.

    Comparing a sliding window of [A, B, C] to [B, C, D]. Since adjacent
    windows always have two elements in common, B+C+D > A+B+C iff D > A.
    */
    let (_, _, _, result) = xs.fold((0, 0, 0, 0), |(p1, p2, p3, acc), x| {
        if p1 != 0 && p1 < x {
            (p2, p3, x, acc + 1)
        } else {
            (p2, p3, x, acc)
        }
    });

    println!("The number of depth increases is {:?}", result);
}
