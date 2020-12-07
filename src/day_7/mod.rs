use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

use anyhow::Result;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug)]
struct Bag {
    description: String,
    held: Vec<(i32, Bag)>,
}

fn bags_from_contains_description(contains_description: &str) -> Vec<(i32, Bag)> {
    if contains_description.contains("no other bags") {
        vec![]
    } else {
        contains_description
            .split(',')
            .map(|item| {
                let mut iter = item.trim_end_matches('.').split_whitespace();
                let count = iter.next().unwrap();
                let description = iter.take(2).join(" ");
                (
                    count.parse().unwrap(),
                    Bag {
                        description,
                        held: vec![],
                    },
                )
            })
            .collect()
    }
}

fn line_to_bag(line: &str) -> Bag {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.*) bags contain (.*)\.").unwrap();
    }
    let captures = RE.captures(line).unwrap();
    let description = captures.get(1).unwrap().as_str();
    let held = bags_from_contains_description(captures.get(2).unwrap().as_str());
    Bag {
        description: description.to_string(),
        held,
    }
}

fn parse_input(input: &str) -> Vec<Bag> {
    input.split('\n').map(line_to_bag).collect()
}

pub fn solve_part_1() -> Result<(), ()> {
    let bags = parse_input(&fs::read_to_string("./inputs/day7.txt").unwrap());
    let containers = determine_containers(&bags, "shiny gold");
    println!("{}", containers.iter().unique().count());
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let bags = parse_input(&fs::read_to_string("./inputs/day7.txt").unwrap());
    let bag_map = hashmap_from_list(bags);
    println!("{}", count_bags_contained(bag_map.get("shiny gold").unwrap(), &bag_map));
    Ok(())
}

fn bags_that_hold_this(bags: &[Bag], description: &str) -> Vec<Bag> {
    bags.iter()
        .filter(|bag| bag.held.iter().any(|(_, bag)| bag.description == description))
        .cloned()
        .collect()
}

fn determine_containers(bags: &[Bag], description: &str) -> Vec<String> {
    let holding_this = bags_that_hold_this(bags, description);
    if !holding_this.is_empty() {
        let mut to_ret = vec![];
        for item in holding_this {
            to_ret.push(item.description.clone());
            to_ret.append(&mut determine_containers(bags, &item.description));
        }
        to_ret.into_iter().unique().collect()
    } else {
        vec![]
    }
}

fn hashmap_from_list(bags: Vec<Bag>) -> HashMap<String, Bag> {
    bags.iter().map(|bag| (bag.description.clone(), bag.clone())).collect()
}

fn count_bags_contained(bag: &Bag, bag_map: &HashMap<String, Bag>) -> usize {
    let here: usize = bag.held.iter().map(|(i, _)| *i as usize).sum();
    let there: usize = bag
        .held
        .iter()
        .map(|(i, bag)| {
            let bag = bag_map.get(&bag.description).unwrap();
            *i as usize * count_bags_contained(bag, bag_map)
        })
        .sum();
    here + there
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_line_to_bag() {
        let correct = Bag {
            description: "drab blue".into(),
            held: vec![
                (
                    4,
                    Bag {
                        description: "striped purple".into(),
                        held: vec![],
                    },
                ),
                (
                    3,
                    Bag {
                        description: "plaid tomato".into(),
                        held: vec![],
                    },
                ),
            ],
        };
        assert_eq!(
            line_to_bag("drab blue bags contain 4 striped purple bags, 3 plaid tomato bags."),
            correct
        );
    }

    #[test]
    fn test_input_part_1() {
        let bags = parse_input(&fs::read_to_string("./test_inputs/day7").unwrap());
        let parents = determine_containers(&bags, "shiny gold");
        let count = parents.iter().unique().count();
        assert_eq!(count, 4)
    }

    #[test]
    fn test_input_part_2() {
        let bags = parse_input(&fs::read_to_string("./test_inputs/day7").unwrap());
        let bag_map = hashmap_from_list(bags);
        assert_eq!(count_bags_contained(bag_map.get("shiny gold").unwrap(), &bag_map), 32);
    }
}
