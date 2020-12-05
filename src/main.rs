mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

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
    const FAILURE_TEXT: &str = "Failed to find the answer";
    match (args.day, args.part) {
        (1, 1) => day_1::solve_part_1().expect(FAILURE_TEXT),
        (1, 2) => day_1::solve_part_2().expect(FAILURE_TEXT),
        (2, 1) => day_2::solve_part_1().expect(FAILURE_TEXT),
        (2, 2) => day_2::solve_part_2().expect(FAILURE_TEXT),
        (3, 1) => day_3::solve_part_1().expect(FAILURE_TEXT),
        (3, 2) => day_3::solve_part_2().expect(FAILURE_TEXT),
        (4, 1) => unimplemented!("This day no work yet, brah."),
        (4, 2) => day_4::solve_part_1().expect(FAILURE_TEXT),
        (5, 1) => day_5::solve_part_1().expect(FAILURE_TEXT),
        (5, 2) => day_5::solve_part_2().expect(FAILURE_TEXT),
        (_, _) => unimplemented!("This day no work yet, brah."),
    };
}
