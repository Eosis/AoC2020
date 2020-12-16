use hashbrown::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::ops::RangeInclusive;

pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("./inputs/day16.txt").unwrap());
    println!("{}", part_1(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("./inputs/day16.txt").unwrap());
    let result = part_2(input);
    println!("{}", result);
    Ok(())
}

fn part_1(problem: Problem) -> u32 {
    problem
        .nearby_tickets
        .iter()
        .flat_map(|ticket| get_failing_values_from_ticket(&problem.criteria, ticket).into_iter())
        .sum()
}

fn part_2(problem: Problem) -> usize {
    let my_ticket = problem.my_ticket.clone();
    let criteria_to_columns = determine_criteria_to_columns(problem);

    let relevant_columns: Vec<_> = criteria_to_columns
        .iter()
        .filter(|(k, _)| (0..=5).contains(*k))
        .map(|(_, v)| v)
        .collect();
    relevant_columns
        .iter()
        .map(|col_idx| my_ticket[**col_idx] as usize)
        .product()
}

#[derive(Debug)]
struct Problem {
    criteria: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>,
    my_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

fn criteria_from_input_line(line: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r".*+: (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    }
    let captures = RE.captures(line).unwrap();
    let min_1 = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let max_1 = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
    let min_2 = captures.get(3).unwrap().as_str().parse::<u32>().unwrap();
    let max_2 = captures.get(4).unwrap().as_str().parse::<u32>().unwrap();
    (min_1..=max_1, min_2..=max_2)
}

fn parse_input(input: &str) -> Problem {
    let mut input_iter = input.split("\n\n");
    let criteria = input_iter.next().unwrap();
    let my_ticket = input_iter.next().unwrap();
    let nearby_tickets = input_iter.next().unwrap();

    let criteria = criteria.split('\n').map(criteria_from_input_line).collect();
    let my_ticket = my_ticket
        .split('\n')
        .nth(1)
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let nearby_tickets = nearby_tickets
        .split('\n')
        .skip(1)
        .map(|x| x.split(',').map(|x| x.parse::<u32>().unwrap()).collect())
        .collect();
    Problem {
        criteria,
        my_ticket,
        nearby_tickets,
    }
}
type TicketRange = RangeInclusive<u32>;

#[inline]
fn check_against_all_criteria(criteria: &[(TicketRange, TicketRange)], value: u32) -> bool {
    criteria
        .iter()
        .any(|(range_0, range_1)| range_0.contains(&value) || range_1.contains(&value))
}

#[inline]
fn get_failing_values_from_ticket(criteria: &[(TicketRange, TicketRange)], ticket: &[u32]) -> Vec<u32> {
    ticket
        .iter()
        .copied()
        .filter(|value| !check_against_all_criteria(criteria, *value))
        .collect()
}

#[inline]
fn check_ticket_validity(criteria: &[(TicketRange, TicketRange)], ticket: &[u32]) -> bool {
    !ticket.iter().any(|value| !check_against_all_criteria(criteria, *value))
}

#[inline]
fn check_all_against_criteria((range_0, range_1): &(TicketRange, TicketRange), values: &[u32]) -> bool {
    values
        .iter()
        .all(|value| range_0.contains(value) || range_1.contains(value))
}

#[inline]
fn get_matching_columns_for_criteria(
    criteria: &(TicketRange, TicketRange),
    columns: &HashMap<usize, Vec<u32>>,
) -> Vec<usize> {
    columns
        .iter()
        .filter(|(_, col)| check_all_against_criteria(&criteria, *col))
        .map(|(i, _)| *i)
        .collect()
}

fn determine_criteria_to_columns(mut problem: Problem) -> HashMap<usize, usize> {
    // Add our ticket to the nearby to get all tickets:
    problem.nearby_tickets.push(problem.my_ticket.clone());
    let all_tickets = &problem.nearby_tickets;
    let valid_tickets = all_tickets
        .iter()
        .filter(|ticket| check_ticket_validity(&problem.criteria, *ticket));
    let valid_tickets_as_cols = convert_tickets_to_cols(valid_tickets, problem.my_ticket.len());
    let mut all_criteria_map: HashMap<usize, (TicketRange, TicketRange)> =
        problem.criteria.iter().cloned().enumerate().collect();
    let mut correct_criteria_map = HashMap::new();
    let mut idx_to_cols: HashMap<usize, Vec<_>> = valid_tickets_as_cols.into_iter().enumerate().collect();

    while !all_criteria_map.is_empty() {
        let criteria_to_matching_columns: HashMap<usize, Vec<usize>> = all_criteria_map
            .iter()
            .map(|(key, criteria)| (*key, get_matching_columns_for_criteria(criteria, &idx_to_cols)))
            .collect();
        if criteria_to_matching_columns
            .iter()
            .filter(|(_, v)| v.len() == 1)
            .count()
            > 1
        {
            println!("More than one column matched. Oh dear");
        }
        let current_match = criteria_to_matching_columns
            .iter()
            .find(|(_, matching)| matching.len() == 1)
            .unwrap();
        let (row_idx, col_idx) = (*current_match.0, current_match.1[0]);
        correct_criteria_map.insert(row_idx, col_idx);
        all_criteria_map.remove(&row_idx);
        idx_to_cols.remove(&col_idx);
    }
    correct_criteria_map
}

fn convert_tickets_to_cols<'a, I: Iterator<Item = &'a Vec<u32>>>(iter: I, len: usize) -> Vec<Vec<u32>> {
    iter.fold(vec![vec![]; len], |mut as_cols, ticket| {
        for (i, value) in ticket.iter().enumerate() {
            as_cols[i].push(*value);
        }
        as_cols
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use hashbrown::HashMap;
    use std::collections::HashSet;

    #[test]
    fn test_invalid_accumulator() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day16").unwrap());
        println!("{:?}", input);
        assert_eq!(part_1(input), 71);
    }

    #[test]
    fn test_ticket_validity() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day16").unwrap());
        assert_eq!(check_ticket_validity(&input.criteria, &input.nearby_tickets[0]), true);
        assert_eq!(check_ticket_validity(&input.criteria, &input.nearby_tickets[1]), false);
        assert_eq!(check_ticket_validity(&input.criteria, &input.nearby_tickets[2]), false);
        assert_eq!(check_ticket_validity(&input.criteria, &input.nearby_tickets[3]), false);
    }

    #[test]
    fn test_determine_criteria_to_columns() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day16_2").unwrap());
        let mut result: HashMap<usize, usize> = HashMap::new();
        result.insert(1, 0);
        result.insert(0, 1);
        result.insert(2, 2);
        assert_eq!(result, determine_criteria_to_columns(input));
    }

    #[test]
    fn test_convert_tickets_to_cols() {
        let input = vec![vec![1, 2, 3], vec![10, 20, 30], vec![100, 200, 300]];
        let correct = vec![vec![1, 10, 100], vec![2, 20, 200], vec![3, 30, 300]];
        assert_eq!(convert_tickets_to_cols(input.iter(), 3), correct);
    }

    #[test]
    fn test_get_matching_columns_for_criteria() {
        let input = vec![vec![1, 2, 3], vec![20, 30, 40], vec![50, 60, 70]];
        let input: HashMap<usize, Vec<u32>> = input.into_iter().enumerate().collect();
        let criteria = (1..=3, 50..=70);
        let correct: HashSet<_> = vec![0, 2].into_iter().collect();
        let result: HashSet<_> = get_matching_columns_for_criteria(&criteria, &input)
            .into_iter()
            .collect();
        assert_eq!(result, correct);
    }
}
