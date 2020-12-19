use hashbrown::HashMap;
use itertools::Itertools;
use regex::Regex;
use std::fs;
use std::str::FromStr;

pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("./inputs/day19.txt").unwrap());
    println!("{}", part_1(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    Ok(())
}

struct Input {
    relations: HashMap<usize, Item>,
    to_check: String,
}

fn part_1(input: Input) -> usize {
    let relations = input.relations;
    let to_check = input.to_check;
    let regex_string = populate_regex_strings(relations).remove(&0).unwrap();
    println!("Regex string is: {}", regex_string);
    let regex_string = [r"\A", &regex_string, r"\z"].join("");
    let re = Regex::from_str(&regex_string).unwrap();
    to_check.split('\n').filter(|line| re.is_match(line)).count()
}

#[allow(dead_code)]
fn part_2(_: HashMap<usize, Item>) -> u64 {
    0
}

#[inline]
// This function is working on the assumption that the required regex strings are already in the map.
fn requirement_to_regex_str(requirement: &[usize], regex_strings: &HashMap<usize, String>) -> String {
    requirement
        .iter()
        .map(|req| regex_strings.get(req).unwrap().clone())
        .join("")
}

// This function is working on the assumption that the required regex strings are already in the map unless it is a raw.
fn item_to_regex_string(item: Item, regex_strings: &HashMap<usize, String>) -> String {
    match item {
        Item::Raw(c) => c.to_string(),
        Item::Requirements(requirements) => [
            "(".to_string(),
            requirements
                .iter()
                .map(|req| requirement_to_regex_str(req, regex_strings))
                .join("|"),
            ")".to_string(),
        ]
        .join(""),
    }
}

fn set_regex_for_key(key: usize, input: &HashMap<usize, Item>, result: &mut HashMap<usize, String>) {
    match input.get(&key).unwrap() {
        Item::Requirements(reqs) => {
            let dependencies: Vec<_> = reqs.iter().flat_map(|req| req.iter()).collect();
            if dependencies.iter().all(|dep| result.contains_key(dep)) {
                let to_insert = item_to_regex_string(Item::Requirements(reqs.clone()), result);
                result.insert(key, to_insert);
            } else {
                let deps_to_set: Vec<_> = dependencies.iter().filter(|k| !result.contains_key(*k)).collect();
                for dep in deps_to_set {
                    if !result.contains_key(dep) {
                        set_regex_for_key(**dep, &input, result);
                    }
                }
                set_regex_for_key(key, &input, result);
            }
        }
        _ => panic!("should only get requirements here"),
    }
}

fn populate_regex_strings(mut input: HashMap<usize, Item>) -> HashMap<usize, String> {
    let mut as_regex_strings = HashMap::new();
    //populate the raws first so there is something to work with.
    let to_prepop: Vec<(usize, Item)> = input
        .iter()
        .filter(|(_, v)| matches!(v, Item::Raw(_)))
        .map(|(k, v)| (*k, v.clone()))
        .collect();
    for (k, raw) in to_prepop {
        as_regex_strings.insert(k, item_to_regex_string(raw, &as_regex_strings));
        input.remove(&k);
    }

    set_regex_for_key(0, &input, &mut as_regex_strings);
    as_regex_strings
}

#[derive(Debug, Clone)]
enum Item {
    Requirements(Vec<Vec<usize>>),
    Raw(char),
}

fn parse_value(input: &str) -> Item {
    let input = input.trim_start();
    if input.starts_with('\"') {
        Item::Raw(input.chars().nth(1).unwrap())
    } else {
        let mut requirements = vec![];
        let mut adding_to = vec![];
        for item in input.split_whitespace() {
            if item.starts_with('|') {
                requirements.push(adding_to);
                adding_to = vec![];
            } else {
                adding_to.push(item.parse::<usize>().unwrap());
            }
        }
        requirements.push(adding_to);
        Item::Requirements(requirements)
    }
}

fn parse_input(input: &str) -> Input {
    let mut iter = input.split("\n\n");
    let relations = parse_relations(iter.next().unwrap());
    let to_check = iter.next().unwrap().to_string();
    Input { relations, to_check }
}

fn parse_relations(input: &str) -> HashMap<usize, Item> {
    input
        .split('\n')
        .map(|line| {
            let mut iter = line.split(':');
            let key = iter.next().unwrap().parse::<usize>().unwrap();
            let value = parse_value(iter.next().unwrap());
            (key, value)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parsing() {
        println!(
            "{:?}",
            parse_input(&fs::read_to_string("./test_inputs/day_19").unwrap()).relations
        );
    }

    #[test]
    fn test_retrieving_regexes() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day_19").unwrap());
        println!("{:?}", input.relations);
        let result = populate_regex_strings(input.relations);
        for item in result.iter().sorted() {
            println!("{:?}", item);
        }
    }

    #[test]
    fn test_part_1() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day_19").unwrap());
        assert_eq!(2, part_1(input));
    }
}
