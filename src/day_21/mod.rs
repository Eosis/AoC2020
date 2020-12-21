use hashbrown::HashMap;
use hashbrown::HashSet;
use itertools::Itertools;
use regex::Regex;
use std::fs;
use std::str::FromStr;

pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("./inputs/day_21.txt").unwrap());
    println!("{}", part_1(input));
    Ok(())
}

fn part_1(input: Input) -> usize {
    let allergens = determine_allergens(input.allergens_to_ingredients, vec![]);
    input.list_of_individual_ingredients.iter().filter(|ingredient| !allergens.contains(ingredient)).count()
}

pub fn solve_part_2() -> Result<(), ()> {
    unimplemented!();
    // let input = parse_input(&fs::read_to_string("./inputs/day19.txt").unwrap());
    // println!("{}", part_2(input));
    Ok(())
}

fn parse_input(input: &str) -> Input {
    let mut allergens_to_ingredients = HashMap::new();
    let mut list_of_individual_ingredients = vec![];
    for (allergens_list, ingredients_set) in input.split('\n')
        .map(|line| {
            let ingredients: String = line.chars().take_while(|c| *c != '(').collect();
            let allergens: String = line.chars().skip_while(|&c| c != '(').take_while(|&c| c != ')').collect();
            let ingredients_set: HashSet<String> = ingredients.trim().split_whitespace().map(str::to_string).collect();
            let mut ingredients_vec: Vec<String> = ingredients.trim().split_whitespace().map(str::to_string).collect();
            list_of_individual_ingredients.append(&mut ingredients_vec);
            let allergens_list: Vec<String> = allergens.trim().split_whitespace().skip(1).map(|value| value.trim_matches(',').to_string()).collect();
            (allergens_list, ingredients_set)
        }) {
        for allergen in allergens_list {
            let entry = allergens_to_ingredients.entry(allergen).or_insert_with(|| vec![]);
            entry.push(ingredients_set.clone());
        }
    }
    Input {
        allergens_to_ingredients,
        list_of_individual_ingredients,
    }
}

fn remove_allergen_ingredient_from_map(map: &mut HashMap<String, Vec<HashSet<String>>>, allergen_ingredient: &str) {
    for (_, v) in map.iter_mut() {
        for set in v.iter_mut() {
            set.remove(allergen_ingredient);
        }
    }
}

fn determine_allergens(mut input: HashMap<String, Vec<HashSet<String>>>, mut current_allergens: Vec<String>) -> Vec<String> {
    if input.len() == 0 {
        return current_allergens;
    } else {
        for (allergen, ingredients) in input.iter().sorted_by_key(|(k, _)| k.clone()) {
            let mut iter = ingredients.iter();
            let first_set = ingredients.iter().next().unwrap().clone();
            let result_of_intersections = iter
                .fold(first_set,
                      |acc, set| acc.intersection(set).cloned().collect());
            if result_of_intersections.len() == 1 {
                //This ingredient is this allergen, fo' sho'. We need to remove it from the other sets.
                let allergen_ingredient = result_of_intersections.iter().next().unwrap();
                current_allergens.push(allergen_ingredient.clone());
                let mut new_input = input.clone();
                new_input.remove(allergen);
                remove_allergen_ingredient_from_map(&mut new_input, &allergen_ingredient);
                return determine_allergens(new_input, current_allergens);
            }
        }
    }
    panic!("Shouldn't get here.");
}

#[derive(Debug, Clone)]
struct Input {
    allergens_to_ingredients: HashMap<String, Vec<HashSet<String>>>,
    list_of_individual_ingredients: Vec<String>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parsing() {
        println!(
            "{:#?}",
            parse_input(&fs::read_to_string("./test_inputs/day21").unwrap())
        );
    }

    #[test]
    fn test_part_1() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day21").unwrap());
        assert_eq!(part_1(input), 5);
    }
}
