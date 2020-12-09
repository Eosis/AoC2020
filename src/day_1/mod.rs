use num::Integer;
use std::fs;

fn get_vals() -> Vec<i32> {
    let input = fs::read_to_string("./inputs/day1.txt").unwrap();
    input.split_whitespace().filter_map(|s| s.parse().ok()).collect()
}

pub fn solve_part_1() -> Result<(), ()> {
    let vals = get_vals();
    println!("Solution: {}", part_one(&vals, 2020).expect("No solution?"));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let vals = get_vals();
    println!("Solution: {}", part_two(&vals).expect("No solution?"));
    Ok(())
}

fn part_one(vals: &[i32], target: i32) -> Option<i32> {
    let mut a = vals[0];
    let mut offset = 1;
    while offset < vals.len() {
        for b in vals[offset..vals.len()].iter() {
            if (a + b) == target {
                return Some(a * b);
            }
        }
        a = vals[offset];
        offset += 1;
    }
    None
}

fn part_two(vals: &[i32]) -> Option<i32> {
    let mut offset = 0;
    while offset < vals.len() - 1 {
        let target = 2020 - vals[offset];
        if let Some(result) = part_one(&vals[(offset + 1)..vals.len()], target) {
            return Some(vals[offset] * result);
        }
        offset += 1
    }
    None
}

use itertools::Itertools;
use std::iter::{Product, Sum};

pub fn combinations_summing_to_n<T: Copy + Integer + Sum<T> + Product<T>>(
    vals: &[T],
    take: usize,
    target: T,
) -> Option<T> {
    vals.iter()
        .copied()
        .combinations(take)
        .find(|combo| combo.iter().copied().sum::<T>() == target)
        .map(|i| i.iter().copied().product())
}

#[cfg(test)]
mod tests {
    use super::*;
    const VALS: [i32; 6] = [1721, 979, 366, 299, 675, 1456];

    #[test]
    fn test_part_one_combo() {
        assert_eq!(combinations_summing_to_n(&VALS, 2, 2020).unwrap(), 514579);
    }

    #[test]
    fn test_part_two_combo() {
        assert_eq!(combinations_summing_to_n(&VALS, 3, 2020).unwrap(), 241861950)
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&VALS, 2020).unwrap(), 514579);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&VALS).unwrap(), 241861950);
    }
}
