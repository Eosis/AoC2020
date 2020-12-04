use std::fs;

pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("inputs/day3.txt").unwrap());
    println!("Solution: {}", part_1(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("inputs/day3.txt").unwrap());
    println!("Solution: {}", part_2(input, &original));
    Ok(())
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let width = input.split('\n').next().unwrap().len();
    let mut slope: Vec<Vec<_>> = vec![vec![]; width];
    for (i, c) in input.chars().filter(|c| *c != '\n').enumerate() {
        slope[i % width].push(c);
    }
    slope
}

fn part_1(input: Vec<Vec<char>>) -> usize {
    original(input, 1, 3)
}

fn part_2(input: Vec<Vec<char>>, work_out: &dyn Fn(Vec<Vec<char>>, usize, usize) -> usize) -> usize {
    [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .iter()
        .copied()
        .map(|(down, right)| work_out(input.clone(), down, right))
        .product::<usize>()
}

fn original(input: Vec<Vec<char>>, down: usize, right: usize) -> usize {
    let mut columns = input.iter().cycle().step_by(right).skip(1);
    let range = 0..(input[0].len());
    range
        .step_by(down)
        .skip(1)
        .map(|i| (&mut columns).next().unwrap()[i])
        .filter(|c| *c == '#')
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_test_input() -> Vec<Vec<char>> {
        parse_input(&fs::read_to_string("test_inputs/day3").unwrap())
    }

    fn ranges(input: Vec<Vec<char>>, down: usize, right: usize) -> usize {
        let height_iter = (0..input[0].len()).step_by(down).skip(1);
        let width_iter = (0..input.len()).cycle().step_by(right).skip(1);
        height_iter
            .zip(width_iter)
            .filter(|(y, x)| input[*x][*y] == '#')
            .count()
    }

    #[test]
    fn test_part_1() {
        let input = read_test_input();
        assert_eq!(part_1(input), 7);
    }

    #[test]
    fn test_part_2_orig() {
        let input = read_test_input();
        assert_eq!(part_2(input, &original), 336);
    }

    #[test]
    fn test_part_2_nouvelle() {
        let input = read_test_input();
        assert_eq!(part_2(input, &ranges), 336);
    }
}
