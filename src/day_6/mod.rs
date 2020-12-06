use anyhow::Result;
use std::collections::HashSet;
use std::fs;

fn parse_input(input: &str) -> Vec<String> {
    input.split("\n\n").map(|s| s.to_string()).collect()
}

pub fn solve_part_1() -> Result<(), ()> {
    println!(
        "{}",
        parse_input(&fs::read_to_string("inputs/day6.txt").unwrap())
            .into_iter()
            .map(|group| group.chars().filter(|c| *c != '\n').collect::<HashSet<char>>().len())
            .sum::<usize>()
    );
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    println!(
        "{}",
        parse_input(&fs::read_to_string("inputs/day6.txt").unwrap())
            .into_iter()
            .map(|x| individual_sets(&x))
            .map(intersection_size)
            .sum::<usize>()
    );
    Ok(())
}

fn individual_sets(group: &str) -> Vec<HashSet<char>> {
    group
        .split('\n')
        .map(|indiv| indiv.chars().collect::<HashSet<char>>())
        .collect()
}

fn intersection_size(sets: Vec<HashSet<char>>) -> usize {
    let mut iter = sets.into_iter();
    iter.next()
        .map(|set| {
            iter.fold(set, |set1, set2| {
                set1.intersection(&set2).copied().collect::<HashSet<char>>()
            })
        })
        .map(|x| x.len())
        .unwrap()
}
