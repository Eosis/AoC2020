use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

struct Entry {
    min: usize,
    max: usize,
    check: char,
    password: String,
}

fn parse_input(input: &str) -> Vec<Entry> {
    input.split('\n').filter_map(line_to_entry).collect()
}

fn line_to_entry(line: &str) -> Option<Entry> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    }
    let captures = RE.captures(line)?;
    let min = captures.get(1)?.as_str();
    let max = captures.get(2)?.as_str();
    let char = captures.get(3)?.as_str();
    let password = captures.get(4)?.as_str();

    Some(Entry {
        min: min.parse::<usize>().ok()?,
        max: max.parse::<usize>().ok()?,
        check: char.chars().next()?,
        password: password.to_string(),
    })
}

fn part_1(input: Vec<Entry>) -> usize {
    input
        .iter()
        .filter(check_part_1_criteria)
        .count()
}

fn part_2(input: Vec<Entry>) -> usize {
    input.iter().filter(check_part_2_criteria).count()
}

#[inline]
fn check_part_1_criteria(entry: &&Entry) -> bool {
    let count = entry.password.chars().filter(|&c| c == entry.check).count();
    (entry.min..=entry.max).contains(&count)
}

#[inline]
fn check_part_2_criteria(entry: &&Entry) -> bool {
    let bytes = entry.password.as_bytes();
    let (idx1, idx2) = (entry.min - 1, entry.max - 1);
    let (first, second) = (bytes[idx1] as char, bytes[idx2] as char);
    let (res1, res2) = (first == entry.check, second == entry.check);
    res1 ^ res2
}

pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("inputs/day2.txt").unwrap());
    println!("Solution: {}", part_1(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("inputs/day2.txt").unwrap());
    println!("Solution: {}", part_2(input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "1-3 a: abcde\n\
                             1-3 b: cdefg\n\
                             2-9 c: ccccccccc\n";
    #[test]
    fn test_part_1() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part_1(input), 2)
    }

    #[test]
    fn test_part_2() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part_2(input), 1)
    }
}
