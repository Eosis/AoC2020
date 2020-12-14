use std::collections::VecDeque;
use std::fs;

fn parse_input(input: &str) -> Vec<u32> {
    input.split('\n').map(|i| i.parse().unwrap()).collect()
}

pub fn solve_part_1() -> Result<(), ()> {
    println!(
        "{}",
        part_1(&mut parse_input(&fs::read_to_string("./inputs/day10.txt").unwrap()))
    );
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    println!(
        "{}",
        part_2(parse_input(&fs::read_to_string("./inputs/day10.txt").unwrap()))
    );
    Ok(())
}

struct MySlicesIterator<'a> {
    items: &'a [u32],
    idx: usize,
}

impl<'a> Iterator for MySlicesIterator<'a> {
    type Item = &'a [u32];
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.items[self.idx..self.items.len()]
            .windows(2)
            .enumerate()
            .find(|(_, slice)| slice[1] - slice[0] == 3);
        match result {
            Some((index_in_other, _)) => {
                let start = self.idx;
                let end = self.idx + index_in_other + 1;
                self.idx = end;
                Some(&self.items[start..end])
            }
            None => {
                if !self.items[self.idx..].is_empty() {
                    let to_send = &self.items[self.idx..];
                    self.idx += self.items[self.idx..].len();
                    Some(to_send)
                } else {
                    None
                }
            }
        }
    }
}

fn ways(items: &[u32]) -> usize {
    let initial = items[0];
    let next_items: Vec<(usize, u32)> = items[0..]
        .iter()
        .copied()
        .enumerate()
        .take(4)
        .map(|(idx, value)| (idx, value - initial))
        .filter(|(_, value)| *value <= 3u32)
        .collect();

    match next_items.len() {
        2..=4 => (1..next_items.len()).map(|i| ways(&items[i..])).sum::<usize>(),
        1 => 1,
        _ => panic!("Wrong slicin'"),
    }
}

fn part_2(mut items: Vec<u32>) -> usize {
    items.sort_unstable();
    let mut items: VecDeque<_> = items.into_iter().collect();
    items.push_front(0);
    items.push_back(items.iter().max().unwrap() + 3);
    let items_slice = items.make_contiguous();
    let iterator = MySlicesIterator {
        items: &items_slice,
        idx: 0,
    };
    iterator.map(|slice| ways(slice)).product()
}

fn part_1(items: &mut [u32]) -> u32 {
    let mut ones = 0;
    let mut threes = 1; // Always one at end
    items.sort_unstable();
    if items[0] == 1 {
        ones += 1
    };
    if items[1] == 3 {
        threes += 1
    };

    for (x, y) in items.windows(2).map(|x| (x[0], x[1])) {
        if y - x == 1 {
            ones += 1;
        } else {
            threes += 1;
        }
    }
    ones * threes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut items = parse_input(&fs::read_to_string("./test_inputs/day10").unwrap());
        assert_eq!(part_1(&mut items), 10 * 22);
    }

    #[test]
    fn test_part_two() {
        let items = parse_input(&fs::read_to_string("./test_inputs/day10_small").unwrap());
        assert_eq!(part_2(items), 8);
        let items = parse_input(&fs::read_to_string("./test_inputs/day10").unwrap());
        assert_eq!(part_2(items), 19_208);
    }

    #[test]
    fn test_slices_iterator() {
        let items = vec![1, 2, 3, 6, 7, 8, 9, 12];
        let mut iter = MySlicesIterator { items: &items, idx: 0 };
        assert_eq!(iter.next().unwrap(), vec![1, 2, 3].as_slice());
        assert_eq!(iter.next().unwrap(), vec![6, 7, 8, 9].as_slice());
        assert_eq!(iter.next().unwrap(), vec![12].as_slice());
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_actual_part_2_answer() {
        let items = parse_input(&fs::read_to_string("./inputs/day10.txt").unwrap());
        assert_eq!(part_2(items), 1_511_207_993_344);
    }

    #[test]
    fn test_example() {
        let items = [1, 2, 3, 4, 5, 6];
        println!("{}", part_2(items.to_vec()))
    }

    #[test]
    fn print_diffs() {
        let mut input = parse_input(&fs::read_to_string("./inputs/day10.txt").unwrap());
        input.sort_unstable();
        let res: Vec<_> = input.windows(2).map(|window| window[1] - window[0]).collect();
        for item in res {
            println!("{}", item);
        }
    }
}
