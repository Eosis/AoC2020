use std::fs;

pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("inputs/day3.txt").unwrap());
    println!("Solution: {}", part_1(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("inputs/day3.txt").unwrap());
    println!("Solution: {}", part_2(input));
    Ok(())
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let width = input.split('\n').next().unwrap().len();
    let mut map: Vec<Vec<_>> = vec![vec![]; width];
    for (i, c) in input.chars().filter(|c| *c != '\n').enumerate() {
        map[i % width].push(c);
    }
    map
}

fn part_1(input: Vec<Vec<char>>) -> usize {
    work_out(input, 1, 3)
}

fn part_2(input: Vec<Vec<char>>) -> usize {
    [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .iter()
        .copied()
        .map(|(down, right)| work_out(input.clone(), down, right))
        .product::<usize>()
}

fn work_out(input: Vec<Vec<char>>, down: usize, right: usize) -> usize {
    let mut columns = input.iter().cycle();
    let col_ref = &mut columns;
    let range = 0..(input[0].len());
    let _: Vec<_> = col_ref.take(right).collect();
    range
        .skip(down)
        .step_by(down)
        .map(|i| {
            let c = col_ref.next().unwrap()[i];
            let _: Vec<_> = col_ref.take(right - 1).collect();
            c
        })
        .filter(|c| *c == '#')
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_test_input() -> Vec<Vec<char>> {
        parse_input(&fs::read_to_string("test_inputs/day3").unwrap())
    }

    #[test]
    fn test_part_1() {
        let input = read_test_input();
        assert_eq!(part_1(input), 7);
    }
    #[test]
    fn test_part_2() {
        let input = read_test_input();
        assert_eq!(part_2(input), 336);
    }
}
