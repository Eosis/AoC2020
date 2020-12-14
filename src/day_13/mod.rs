use std::fs;

fn parse_input(input: &str) -> Problem {
    let current_time = input.split("\n").next().unwrap().parse().unwrap();
    let available_freqs = input
        .split("\n")
        .nth(1)
        .map(|value_str| {
            value_str
                .split(",")
                .map(|freq| if freq == "x" { 0 } else { freq.parse().unwrap() })
                .collect()
        })
        .unwrap();
    Problem {
        current_time,
        available_freqs,
    }
}

pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("inputs/day13.txt").unwrap());
    println!("{}", part_1(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("inputs/day13.txt").unwrap());
    println!("{}", part_2(input));
    Ok(())
}

fn part_1(problem: Problem) -> u64 {
    let nearest_freq = problem
        .available_freqs
        .iter()
        .filter(|x| **x != 0)
        .min_by_key(|freq| **freq - (problem.current_time % **freq))
        .unwrap();
    let waiting = nearest_freq - (problem.current_time % nearest_freq);
    nearest_freq * waiting
}

fn get_modulo_expectation((place, value): (usize, &u64)) -> (u64, u64) {
    let place = place as u64;
    if *value > place {
        ((*value - place) % *value, *value)
    } else {
        let mut place = place;
        let mut value = *value;
        while place > value {
            place -= value;
        }
        (value - place, value)
    }
}

fn make_constraints(buses: &[u64]) -> Vec<(u64, u64)> {
    buses
        .iter()
        .enumerate()
        .filter(|(i, n)| **n != 0)
        .map(get_modulo_expectation)
        .collect()
}

fn part_2(problem: Problem) -> u64 {
    let constraints = make_constraints(&problem.available_freqs);
    sieving_search(constraints)
}

struct Problem {
    current_time: u64,
    available_freqs: Vec<u64>,
}

fn sieving_search(mut list: Vec<(u64, u64)>) -> u64 {
    list.sort_by_key(|(_, modulo)| *modulo);
    let mut inverse_mod_order = list.into_iter().rev();
    let (expected, modulo) = inverse_mod_order.next().unwrap();
    let mut current_number = expected;
    let mut current_increment = modulo;
    for (expected, modulo) in inverse_mod_order {
        let mut found = false;
        while !found {
            current_number += current_increment;
            if current_number % modulo == expected {
                found = true;
                current_increment = current_increment * modulo;
            }
        }
    }
    current_number
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input_str = "939\n\
            7,13,x,x,59,x,31,19";
        let input = parse_input(input_str);
        assert_eq!(part_1(input), 295);
    }

    fn print_useful(factor: u64, divisor: u64) {
        println!("Table for {} % {}", factor, divisor);
        for i in (1..=divisor) {
            println!("({}, {}, {})", i, factor * i, (factor * i) % divisor);
        }
    }

    #[test]
    #[ignore]
    fn print_some() {
        let things: Vec<_> = vec![(13, 17), (19, 17), (19, 13)];
        for (factor, divisor) in &things {
            print_useful(*factor, *divisor);
        }
    }

    #[test]
    fn test_sieving() {
        let list = vec![(0, 3), (3, 4), (4, 5)];
        assert_eq!(sieving_search(list), 39);

        let list = vec![(0, 3), (2, 7), (6, 11), (3, 13)];
        assert_eq!(sieving_search(list), 2382);
    }

    #[test]
    #[ignore]
    fn quick_check() {
        let list = vec![(0, 3), (2, 7), (6, 11), (3, 13)];

        for i in 0.. {
            if list.iter().all(|(expected, modulo)| i % *modulo == *expected) {
                println!("It was {} for these", i);
                break;
            }
        }
    }

    #[test]
    fn test_make_constraints() {
        let input_str = "939\n\
            7,x,x,x,x,x,x,5";
        let expected_result = vec![(0, 7), (3, 5)];
        let problem = parse_input(input_str);
        let buses = problem.available_freqs;
        assert_eq!(make_constraints(&buses), expected_result);
    }

    #[test]
    fn test_examples_part_2() {
        let input_str = "111\n\
        17,x,13,19";
        assert_eq!(part_2(parse_input(input_str)), 3417);

        let input_str = "111\n\
        1789,37,47,1889";
        assert_eq!(part_2(parse_input(input_str)), 1202161486);
    }
}
