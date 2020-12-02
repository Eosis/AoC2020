mod day_1;
mod day_2;

use argh::FromArgs;

#[derive(FromArgs)]
/// Rup's Advent of Code 2020
struct Args {
    /// day that we are doing the puzzle for.
    #[argh(positional)]
    day: usize,

    #[argh(positional)]
    /// part of the puzzle to do.
    part: usize,
}

fn main() {
    let args: Args = argh::from_env();
    match (args.day, args.part) {
        (1, 1) => day_1::solve_part_1().expect("Failed to find the answer"),
        (1, 2) => day_1::solve_part_2().expect("Failed to find the answer"),
        (2, 1) => day_2::solve_part_1().expect("Failed_to_find the answer"),
        (2, 2) => day_2::solve_part_2().expect("Failed_to_find the answer"),
        (_, _) => unimplemented!("This day no work yet, brah."),
    };
}
