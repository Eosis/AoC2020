use anyhow::Result;
use std::collections::{HashMap, VecDeque};
use std::fs;

fn parse_input(input: &str) -> Vec<u64> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("./inputs/day15.txt").unwrap());
    println!("{}", part_1(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("./inputs/day15.txt").unwrap());
    println!("{}", part_2(input));
    Ok(())
}

fn part_1(input: Vec<u64>) -> u64 {
    find_nth_in_sequence(&input, 2020)
}

fn part_2(input: Vec<u64>) -> u64 {
    find_nth_in_sequence_efficient(&input, 30000000)
}

fn find_nth_in_sequence(input: &[u64], n: usize) -> u64 {
    let mut sequence = input.to_vec();
    let starting_turn = input.len() + 1;

    for turn in starting_turn..=n {
        let comparing = sequence.last().unwrap();
        let to_push = sequence[0..sequence.len() - 1]
            .iter()
            .enumerate()
            .filter(|(_, value)| **value == *comparing)
            .max_by_key(|(i, _)| *i)
            .map(|(most_recent_idx, _)| (turn - 1 - (most_recent_idx + 1)))
            .unwrap_or(0);
        sequence.push(to_push as u64);
        // if turn % 100_000 == 0 { println!("Currently on turn {}", turn) }
    }
    *sequence.last().unwrap()
}

type ValuesToTurnsMap = HashMap<u64, VecDeque<usize>>;
fn handle_if_in_map(locations_map: &mut ValuesToTurnsMap, looking_for: u64, turn: usize) -> u64 {
    let turn_list = locations_map.get_mut(&looking_for).unwrap();

    // This only occurs when it was the last value entered, so there is no previous occurrence
    let to_insert = if turn_list.len() == 1 {
        (0, turn)
    } else {
        // if the list is of length > 1,  add or alter a key in the map with value (turn  - 1  - list[length-2])
        let new_value = (turn - 1 - turn_list[0]) as u64;
        (new_value, turn)
    };
    add_or_append_to_key(locations_map, to_insert.0, to_insert.1)
}

fn add_or_append_to_key(locations_map: &mut ValuesToTurnsMap, key: u64, turn: usize) -> u64 {
    let current = locations_map.entry(key).or_insert_with(|| VecDeque::with_capacity(2));
    if current.len() < 2 {
        current.push_back(turn);
    } else {
        current.pop_front();
        current.push_back(turn);
    }
    key
}

fn find_nth_in_sequence_efficient(input: &[u64], n: usize) -> u64 {
    let mut locations_map: ValuesToTurnsMap = ValuesToTurnsMap::new();
    for (turn, value) in input.iter().enumerate() {
        locations_map.entry(*value).or_insert_with(|| {
            let mut value = VecDeque::with_capacity(2);
            value.push_back(turn + 1);
            value
        });
    }
    let starting_turn = input.len() + 1;
    let mut looking_for = *input.last().unwrap();
    for turn in starting_turn..=n {
        looking_for = if locations_map.get(&looking_for).is_some() {
            handle_if_in_map(&mut locations_map, looking_for, turn)
        } else {
            add_or_append_to_key(&mut locations_map, 0, turn)
        }
    }
    looking_for
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simple() {
        let input = "0,3,6";
        let input = parse_input(input);
        assert_eq!(find_nth_in_sequence(&input, 4), 0);
        assert_eq!(find_nth_in_sequence(&input, 5), 3);
        assert_eq!(find_nth_in_sequence(&input, 6), 3);
        assert_eq!(find_nth_in_sequence(&input, 7), 1);
        assert_eq!(find_nth_in_sequence(&input, 8), 0);
        assert_eq!(find_nth_in_sequence(&input, 9), 4);
        assert_eq!(find_nth_in_sequence(&input, 10), 0);
        assert_eq!(find_nth_in_sequence(&input, 2020), 436);
    }

    #[test]
    fn test_efficient() {
        let input = "0,3,6";
        let input = parse_input(input);
        assert_eq!(find_nth_in_sequence_efficient(&input, 4), 0);
        assert_eq!(find_nth_in_sequence_efficient(&input, 5), 3);
        assert_eq!(find_nth_in_sequence_efficient(&input, 6), 3);
        assert_eq!(find_nth_in_sequence_efficient(&input, 7), 1);
        assert_eq!(find_nth_in_sequence_efficient(&input, 8), 0);
        assert_eq!(find_nth_in_sequence_efficient(&input, 9), 4);
        assert_eq!(find_nth_in_sequence_efficient(&input, 10), 0);
        assert_eq!(find_nth_in_sequence_efficient(&input, 2020), 436);
    }

    #[test]
    #[ignore]
    fn test_long_example() {
        let input = "0,3,6";
        let input = parse_input(input);
        assert_eq!(find_nth_in_sequence_efficient(&input, 30000000), 175594);
    }
}
