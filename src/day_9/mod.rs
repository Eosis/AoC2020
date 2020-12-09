use std::fs;

fn parse_input(input: &str) -> Vec<usize> {
    input.split('\n').map(|i| i.parse::<usize>().unwrap()).collect()
}

pub fn solve_part_1() -> Result<(), ()> {
    let items = parse_input(&fs::read_to_string("./inputs/day9.txt").unwrap());
    println!("{}", part_1(&items, 25));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let items = parse_input(&fs::read_to_string("./inputs/day9.txt").unwrap());
    println!("{}", part_2(&items, 776_203_571));
    Ok(())
}

use crate::day_1::combinations_summing_to_n;
fn part_1(items: &[usize], window_size: usize) -> usize {
    for win in items.windows(window_size + 1) {
        if combinations_summing_to_n(&win[0..window_size], 2, win[window_size]).is_none() {
            return win[window_size];
        }
    }
    panic!("Didn't find any!");
}

fn part_2(items: &[usize], target: usize) -> usize {
    for window_size in 2..=items.len() {
        if let Some(x) = items
            .windows(window_size)
            .find(|window| window.iter().sum::<usize>() == target)
            .map(|window| window.iter().min().unwrap() + window.iter().max().unwrap())
        {
            return x;
        }
    }
    panic!("Didn't find any!");
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let items = parse_input(&fs::read_to_string("./test_inputs/day9").unwrap());
        assert_eq!(part_1(&items, 5), 127);
    }

    #[test]
    fn test_part_two() {
        let items = parse_input(&fs::read_to_string("./test_inputs/day9").unwrap());
        assert_eq!(part_2(&items, 127), 62);
    }
}
