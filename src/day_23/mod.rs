mod linked_list;

use crate::day_23::linked_list::print_list_items;
use hashbrown::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

pub fn solve_part_1() -> Result<(), ()> {
    part_1("562893147");
    Ok(())
}

fn part_1(input: &str) {
    let problem = parse_input_part_1(input);
    let result = run_game_as_list(problem, 100);
    print_list_items(Rc::clone(result.get(&1).unwrap()), 9);
}

pub fn solve_part_2() -> Result<(), ()> {
    part_2("562893147");
    Ok(())
}

fn part_2(mut input: &str) {
    let problem = parse_input_part_2(input);
    let result = run_game_as_list(problem, 10_000_000);
    print_list_items(Rc::clone(result.get(&1).unwrap()), 4);
}

struct Problem {
    starting_label: u32,
    max_value: u32,
    list: linked_list::List<u32>,
    map: HashMap<u32, Rc<RefCell<linked_list::Node<u32>>>>,
}

fn str_to_vec(input: &str) -> Vec<u32> {
    input.chars().filter_map(|c| c.to_digit(10)).collect()
}

fn parse_input_part_1(input: &str) -> Problem {
    let original = str_to_vec(input);
    let starting_label = original[0];
    let max_value = original.iter().max().copied().unwrap();
    let (list, map) = setup_list_with_map_smaller(original);
    Problem {
        starting_label,
        max_value,
        list,
        map,
    }
}

fn parse_input_part_2(input: &str) -> Problem {
    let original = str_to_vec(input);
    let starting_label = original[0];
    let (list, map) = setup_list_with_map_larger(original);
    Problem {
        starting_label,
        max_value: 1_000_000,
        list,
        map,
    }
}

fn get_new_label(current_label: u32, lifted: &[u32], max_value: u32) -> u32 {
    let mut new_label = current_label - 1;
    while new_label < 1 || lifted.contains(&new_label) {
        if new_label == 0 {
            new_label = max_value;
        } else {
            new_label -= 1;
        }
    }
    new_label
}

fn setup_list_with_map_smaller(
    input: Vec<u32>,
) -> (
    linked_list::List<u32>,
    HashMap<u32, Rc<RefCell<linked_list::Node<u32>>>>,
) {
    use linked_list::List;
    use linked_list::Node;
    let mut list: List<u32> = List::new();
    let mut map: HashMap<u32, Rc<RefCell<Node<u32>>>> = HashMap::new();
    // Need to join the end of the list:
    let head = list.push_back(input[0]);
    map.insert(input[0], Rc::clone(&head));

    for item in &input[1..input.len() - 1] {
        let to_insert = list.push_back(*item);
        map.insert(*item, Rc::clone(&to_insert));
    }

    let tail = list.push_back(input.last().unwrap().clone());
    map.insert(input.last().unwrap().clone(), Rc::clone(&tail));
    tail.borrow_mut().set_next(&head);

    (list, map)
}

fn setup_list_with_map_larger(
    input: Vec<u32>,
) -> (
    linked_list::List<u32>,
    HashMap<u32, Rc<RefCell<linked_list::Node<u32>>>>,
) {
    use linked_list::List;
    use linked_list::Node;
    let mut list: List<u32> = List::new();
    let mut map: HashMap<u32, Rc<RefCell<Node<u32>>>> = HashMap::new();
    // Need to join the end of the list:
    let head = list.push_back(input[0]);
    map.insert(input[0], Rc::clone(&head));

    for item in &input[1..] {
        let to_insert = list.push_back(*item);
        map.insert(*item, Rc::clone(&to_insert));
    }

    for item in (input.iter().max().unwrap() + 1)..1000000 {
        let to_insert = list.push_back(item);
        map.insert(item, Rc::clone(&to_insert));
    }

    let tail = list.push_back(1000000);
    map.insert(1000000, Rc::clone(&tail));
    tail.borrow_mut().set_next(&head);

    (list, map)
}

fn get_values_of_removed(node: Rc<RefCell<linked_list::Node<u32>>>) -> [u32; 3] {
    let mut current_node = node;
    let mut result = [0, 0, 0];
    for i in 0..3 {
        result[i] = current_node.borrow().elem.clone();
        let next_node = {
            let borrowed_current = current_node.borrow();
            Rc::clone(borrowed_current.next.as_ref().unwrap())
        };
        current_node = next_node;
    }
    result
}

fn run_game_as_list(problem: Problem, moves: usize) -> HashMap<u32, Rc<RefCell<linked_list::Node<u32>>>> {
    use linked_list::Node;
    let Problem {
        map,
        list,
        starting_label: mut current_label,
        max_value,
    } = problem;

    for i in 0..moves {
        if i % 1000000 == 0 {
            println!("at {}", i);
        }
        let current_cup = Rc::clone(map.get(&current_label).unwrap());
        let removed = Node::take_three_out(Rc::clone(&current_cup));
        let values_of_removed: [u32; 3] = get_values_of_removed(Rc::clone(&removed));
        let destination_label = get_new_label(current_label, &values_of_removed, max_value);
        let insert_after = Rc::clone(map.get(&destination_label).unwrap());
        insert_after.borrow_mut().insert_after(removed);
        current_label = {
            let label = current_cup.borrow().next.as_ref().unwrap().borrow().elem.clone();
            label
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        part_1("389125467");
    }

    #[test]
    fn test_part_2() {
        part_2("389125467");
    }

    // #[test]
    // fn test_run_game_a_bit() {
    //     let input = parse_input_part_2("389125467");
    //     run_game(input, 10);
    // }
}
