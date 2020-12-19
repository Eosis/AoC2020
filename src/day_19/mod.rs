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
    let input = parse_input(&fs::read_to_string("./inputs/day19.txt").unwrap());
    println!("{}", part_2(input));
    Ok(())
}

struct Input {
    relations: HashMap<usize, Item>,
    to_check: String,
}

fn part_1(input: Input) -> usize {
    let relations = input.relations;
    let to_check = input.to_check;
    let regex_string = populate_regex_strings(relations, 1).remove(&0).unwrap();
    let regex_string = [r"\A", &regex_string, r"\z"].join("");
    println!("Regex string is: {}", regex_string);
    let re = Regex::from_str(&regex_string).unwrap();
    to_check.split('\n').filter(|line| re.is_match(line)).count()
}

#[allow(dead_code)]
fn part_2(input: Input) -> usize {
    let relations = input.relations;
    let mut to_check: HashMap<usize, String> = input.to_check.split('\n').map(str::to_string).enumerate().collect();
    let mut matched = 0;
    for n in 1..=50 {
        println!("Run {}", n);
        let regex_string = populate_regex_strings(relations.clone(), n).remove(&0).unwrap();
        let regex_string = [r"\A", &regex_string, r"\z"].join("");
        let re = Regex::from_str(&regex_string).unwrap();
        let mut to_remove = vec![];
        for (k, val) in to_check.iter() {
            if re.is_match(val) {
                to_remove.push(*k);
            }
        }
        matched += to_remove.len();
        for k in to_remove {
            to_check.remove(&k);
        }
    }
    matched
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

fn set_regex_for_key_given_requirements(
    key: usize,
    reqs: Vec<Vec<usize>>,
    input: &HashMap<usize, Item>,
    result: &mut HashMap<usize, String>,
    times: usize,
) {
    let dependencies: Vec<_> = reqs.iter().flat_map(|req| req.iter()).collect();
    if dependencies.iter().all(|dep| result.contains_key(dep)) {
        let to_insert = item_to_regex_string(Item::Requirements(reqs.clone()), result);
        result.insert(key, to_insert);
    } else {
        let deps_to_set: Vec<_> = dependencies.iter().filter(|k| !result.contains_key(*k)).collect();
        for dep in deps_to_set {
            if !result.contains_key(dep) {
                set_regex_for_key(**dep, &input, result, times);
            }
        }
        set_regex_for_key(key, &input, result, times);
    }
}

fn set_regex_for_8(input: &HashMap<usize, Item>, result: &mut HashMap<usize, String>) {
    set_regex_for_key(42, input, result, 0);
    // Now have all required for 8.
    let regex_string = ["(", result.get(&42).unwrap(), "+", ")"].join("");
    result.insert(8, regex_string);
}

fn set_regex_for_11(input: &HashMap<usize, Item>, result: &mut HashMap<usize, String>, times: usize) {
    set_regex_for_key(42, input, result, 0);
    set_regex_for_key(31, input, result, 0);
    // Now have all required for 11.
    let regex_string = [
        "(",
        result.get(&42).unwrap(),
        &format!("{{{}}}", times),
        result.get(&31).unwrap(),
        &format!("{{{}}}", times),
        ")",
    ]
    .join("");
    result.insert(11, regex_string);
}

fn set_regex_for_key(key: usize, input: &HashMap<usize, Item>, result: &mut HashMap<usize, String>, times: usize) {
    match input.get(&key).unwrap() {
        Item::Requirements(reqs) => {
            if key == 8 {
                set_regex_for_8(input, result);
            } else if key == 11 {
                set_regex_for_11(input, result, times);
            } else {
                set_regex_for_key_given_requirements(key, reqs.clone(), input, result, times);
            }
        }
        _ => panic!("should only get requirements here"),
    }
}

fn populate_regex_strings(mut input: HashMap<usize, Item>, times: usize) -> HashMap<usize, String> {
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

    set_regex_for_key(0, &input, &mut as_regex_strings, times);
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
        let result = populate_regex_strings(input.relations, 0);
        for item in result.iter().sorted() {
            println!("{:?}", item);
        }
    }

    #[test]
    fn test_part_1() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day_19").unwrap());
        assert_eq!(2, part_1(input));
    }

    #[test]
    fn test_part_2() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day_19_2").unwrap());
        assert_eq!(12, part_2(input))
    }

    #[test]
    fn print_new_dependencies_part_2() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day_19_part_2_reqs").unwrap());
        let result = populate_regex_strings(input.relations, 0);
        for (k, v) in result.iter().sorted_by_key(|(k, _)| **k) {
            println!("{}: {}", k, v);
        }
    }

    #[test]
    fn find_items_which_depend_on_8_or_11() {
        let input = parse_input(&fs::read_to_string("./inputs/day19.txt").unwrap());
        let result = populate_regex_strings(input.relations, 0);
        for item in result.iter().sorted() {
            println!("{:?}", item);
        }
    }
}
