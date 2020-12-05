use itertools::Itertools;
use std::fs;

use anyhow::Result;

pub fn solve_part_1() -> Result<(), ()> {
    println!(
        "{}",
        parse_input(fs::read_to_string("inputs/day5.txt").unwrap())
            .iter()
            .map(Seat::generate_id)
            .max()
            .unwrap()
    );
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let mut seats = parse_input(fs::read_to_string("inputs/day5.txt").unwrap());
    seats.sort();
    let result = seats
        .iter()
        .tuple_windows()
        .find(|(first, second)| first.generate_id() + 1 != second.generate_id())
        .map(|(first, _)| Seat::from_id(first.generate_id() + 1).generate_id())
        .unwrap();
    println!("{}", result);
    Ok(())
}

fn parse_input(input: String) -> Vec<Seat> {
    input.split('\n').filter_map(|item| Seat::from_str(item).ok()).collect()
}

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq)]
struct Seat {
    column: u32,
    row: u32,
}

impl Seat {
    fn from_str(rep: &str) -> anyhow::Result<Seat> {
        let column = &rep[0..7]
            .chars()
            .fold(0, |acc, c| if c == 'B' { (acc << 1) | 1 } else { acc << 1 });
        let row = &rep[7..10]
            .chars()
            .fold(0, |acc, c| if c == 'R' { (acc << 1) | 1 } else { acc << 1 });
        Ok(Seat {
            column: *column,
            row: *row,
        })
    }

    fn generate_id(&self) -> u32 {
        self.column << 3 | self.row
    }

    fn from_id(num: u32) -> Seat {
        Seat {
            column: num >> 3,
            row: num & 0b111,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let reps = ["BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"];
        let correct = vec![
            (Seat { row: 7, column: 70 }, 567),
            (Seat { row: 7, column: 14 }, 119),
            (Seat { row: 4, column: 102 }, 820),
        ];
        let result: Vec<_> = reps
            .iter()
            .map(|rep| {
                let seat = Seat::from_str(rep).unwrap();
                (Seat::from_str(rep).unwrap(), seat.generate_id())
            })
            .collect();
        assert_eq!(result, correct);
    }
}
