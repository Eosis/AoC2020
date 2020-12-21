mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_2;
mod day_20;
mod day_21;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

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
use anyhow::Result;
fn main() -> Result<()> {
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
        (6, 1) => day_6::solve_part_1().expect(FAILURE_TEXT),
        (6, 2) => day_6::solve_part_2().expect(FAILURE_TEXT),
        (7, 1) => day_7::solve_part_1().expect(FAILURE_TEXT),
        (7, 2) => day_7::solve_part_2().expect(FAILURE_TEXT),
        (8, 1) => day_8::solve_part_1().expect(FAILURE_TEXT),
        (8, 2) => day_8::solve_part_2().expect(FAILURE_TEXT),
        (9, 1) => day_9::solve_part_1().expect(FAILURE_TEXT),
        (9, 2) => day_9::solve_part_2().expect(FAILURE_TEXT),
        (10, 1) => day_10::solve_part_1().expect(FAILURE_TEXT),
        (10, 2) => day_10::solve_part_2().expect(FAILURE_TEXT),
        (11, 1) => day_11::solve_part_1().expect(FAILURE_TEXT),
        (11, 2) => day_11::solve_part_2().expect(FAILURE_TEXT),
        (12, 1) => day_12::solve_part_1().expect(FAILURE_TEXT),
        (12, 2) => day_12::solve_part_2().expect(FAILURE_TEXT),
        (13, 1) => day_13::solve_part_1().expect(FAILURE_TEXT),
        (13, 2) => day_13::solve_part_2().expect(FAILURE_TEXT),
        (14, 1) => day_14::solve_part_1().expect(FAILURE_TEXT),
        (14, 2) => day_14::solve_part_2().expect(FAILURE_TEXT),
        (15, 1) => day_15::solve_part_1().expect(FAILURE_TEXT),
        (15, 2) => day_15::solve_part_2().expect(FAILURE_TEXT),
        (16, 1) => day_16::solve_part_1().expect(FAILURE_TEXT),
        (16, 2) => day_16::solve_part_2().expect(FAILURE_TEXT),
        (17, 1) => unimplemented!("This day no work yet, brah."),
        (17, 2) => day_17::solve_part_2().expect(FAILURE_TEXT),
        (18, 1) => day_18::solve_part_1().expect(FAILURE_TEXT),
        (18, 2) => day_18::solve_part_2().expect(FAILURE_TEXT),
        (19, 1) => day_19::solve_part_1().expect(FAILURE_TEXT),
        (19, 2) => day_19::solve_part_2().expect(FAILURE_TEXT),
        (20, 1) => day_20::solve_part_1().expect(FAILURE_TEXT),
        (20, 2) => day_20::solve_part_2().expect(FAILURE_TEXT),
        (21, 1) => day_21::solve_part_1().expect(FAILURE_TEXT),
        (21, 2) => day_21::solve_part_2().expect(FAILURE_TEXT),
        (_, _) => unimplemented!("This day no work yet, brah."),
    };
    Ok(())
}
