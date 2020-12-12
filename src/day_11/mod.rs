use std::fs;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

impl Tile {
    fn from_char(c: char) -> Self {
        use Tile::{Empty, Floor, Occupied};
        match c {
            'L' => Empty,
            '#' => Occupied,
            '.' => Floor,
            _ => panic!("Unrecognised character in parsing to Tile"),
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .split('\n')
        .map(|line| line.chars().map(Tile::from_char).collect())
        .collect()
}

//    const row_max = board.len() - 1;
//     const col_max = board[0].len() - 1;
//
//     let iter_y = match (y, x) {
//         (0, x) => (0..=1),
//         (row_max, x) => (-1i32..=0),
//         (y, x) => (-1i32..=1),
//     };
//     let iter_x = match (y, x) {
//         (y, 0) => (0..=1),
//         (y, col_max) => (-1i32..=0),
//         (y, x) => (-1i32..=1),
//     };

// empty seat-> no adjacent  -> occupied
// occupied -> >= 4 adjacent occupied -> empty
// else -> Same.

fn new_tile_part_1((y, x): (usize, usize), board: &[Vec<Tile>]) -> Tile {
    match board[y][x] {
        Tile::Empty => {
            if count_adjacent_occupied((y, x), board) == 0 {
                Tile::Occupied
            } else {
                Tile::Empty
            }
        }
        Tile::Occupied => {
            if count_adjacent_occupied((y, x), board) >= 4 {
                Tile::Empty
            } else {
                Tile::Occupied
            }
        }
        Tile::Floor => Tile::Floor,
    }
}

fn new_tile_part_2((y, x): (usize, usize), board: &[Vec<Tile>]) -> Tile {
    match board[y][x] {
        Tile::Empty => {
            if count_visible_occupied((y, x), board) == 0 {
                Tile::Occupied
            } else {
                Tile::Empty
            }
        }
        Tile::Occupied => {
            if count_visible_occupied((y, x), board) >= 5 {
                Tile::Empty
            } else {
                Tile::Occupied
            }
        }
        Tile::Floor => Tile::Floor,
    }
}

fn iterate_board(board: &[Vec<Tile>], new_tile: &dyn Fn((usize, usize), &[Vec<Tile>]) -> Tile) -> Vec<Vec<Tile>> {
    let mut new_board = board.to_owned();
    for y in 0..new_board.len() {
        for x in 0..new_board[0].len() {
            new_board[y][x] = new_tile((y, x), &board)
        }
    }
    new_board
}

fn count_adjacent_occupied((y, x): (usize, usize), board: &[Vec<Tile>]) -> usize {
    let max_row = (board.len() - 1) as i32;
    let max_column = (board[0].len() - 1) as i32;
    let incs: Vec<(i32, i32)> = {
        let (y, x) = (y as i32, x as i32);
        let mut incs = vec![];
        for y_inc in -1..=1 {
            for x_inc in -1..=1 {
                incs.push((y_inc, x_inc));
            }
        }
        incs.into_iter()
            .filter(|(y_inc, _)| !(*y_inc + y < 0 || *y_inc + y > max_row))
            .filter(|(_, x_inc)| !(*x_inc + x < 0 || *x_inc + x > max_column))
            .filter(|(y_inc, x_inc)| !(*y_inc == 0 && *x_inc == 0))
            .collect()
    };
    incs.iter()
        .filter(|(y_inc, x_inc)| board[(y as i32 + y_inc) as usize][(x as i32 + x_inc) as usize] == Tile::Occupied)
        .count()
}

fn count_north((y, x): (usize, usize), board: &[Vec<Tile>]) -> usize {
    let first_vis = (1 as usize..)
        .take_while(|inc| (y as i32) - *inc as i32 >= 0)
        .take_while(|inc| board.get(y - *inc).is_some())
        .map(|inc| board[y - inc][x])
        .find(|t| matches!(t, Tile::Occupied | Tile::Empty));
    match first_vis {
        Some(Tile::Empty) | None => 0,
        Some(Tile::Occupied) => 1,
        _ => panic!("Unexpected!"),
    }
}

fn count_south((y, x): (usize, usize), board: &[Vec<Tile>]) -> usize {
    let first_vis = (1 as usize..)
        .take_while(|inc| board.get(y + *inc).is_some())
        .map(|inc| board[y + inc][x])
        .find(|t| matches!(t, Tile::Occupied | Tile::Empty));
    match first_vis {
        Some(Tile::Empty) | None => 0,
        Some(Tile::Occupied) => 1,
        _ => panic!("Unexpected!"),
    }
}

fn count_east((y, x): (usize, usize), board: &[Vec<Tile>]) -> usize {
    let first_vis = (1 as usize..)
        .take_while(|inc| board[y].get(x + *inc).is_some())
        .map(|inc| board[y][x + inc])
        .find(|t| matches!(t, Tile::Occupied | Tile::Empty));
    match first_vis {
        Some(Tile::Empty) | None => 0,
        Some(Tile::Occupied) => 1,
        _ => panic!("Unexpected!"),
    }
}

fn count_west((y, x): (usize, usize), board: &[Vec<Tile>]) -> usize {
    let first_vis = (1 as usize..)
        .take_while(|inc| (x as i32) - *inc as i32 >= 0)
        .take_while(|inc| board[y].get(x - *inc).is_some())
        .map(|inc| board[y][x - inc])
        .find(|t| matches!(t, Tile::Occupied | Tile::Empty));
    match first_vis {
        Some(Tile::Empty) | None => 0,
        Some(Tile::Occupied) => 1,
        _ => panic!("Unexpected!"),
    }
}

fn count_north_east((y, x): (usize, usize), board: &[Vec<Tile>]) -> usize {
    let first_vis = (1 as usize..)
        .take_while(|inc| (y as i32) - *inc as i32 >= 0)
        .take_while(|inc| board.get(y - *inc).is_some() && board[y - *inc].get(x + *inc).is_some())
        .map(|inc| board[y - inc][x + inc])
        .find(|t| matches!(t, Tile::Occupied | Tile::Empty));
    match first_vis {
        Some(Tile::Empty) | None => 0,
        Some(Tile::Occupied) => 1,
        _ => panic!("Unexpected!"),
    }
}

fn count_north_west((y, x): (usize, usize), board: &[Vec<Tile>]) -> usize {
    let first_vis = (1 as usize..)
        .take_while(|inc| (y as i32) - *inc as i32 >= 0 && (x as i32) - *inc as i32 >= 0)
        .take_while(|inc| board.get(y - *inc).is_some() && board[y - *inc].get(x - *inc).is_some())
        .map(|inc| board[y - inc][x - inc])
        .find(|t| matches!(t, Tile::Occupied | Tile::Empty));
    match first_vis {
        Some(Tile::Empty) | None => 0,
        Some(Tile::Occupied) => 1,
        _ => panic!("Unexpected!"),
    }
}

fn count_south_east((y, x): (usize, usize), board: &[Vec<Tile>]) -> usize {
    let first_vis = (1 as usize..)
        .take_while(|inc| board.get(y + *inc).is_some() && board[y + *inc].get(x + *inc).is_some())
        .map(|inc| board[y + inc][x + inc])
        .find(|t| matches!(t, Tile::Occupied | Tile::Empty));
    match first_vis {
        Some(Tile::Empty) | None => 0,
        Some(Tile::Occupied) => 1,
        _ => panic!("Unexpected!"),
    }
}

fn count_south_west((y, x): (usize, usize), board: &[Vec<Tile>]) -> usize {
    let first_vis = (1 as usize..)
        .take_while(|inc| (x as i32) - *inc as i32 >= 0)
        .take_while(|inc| board.get(y + *inc).is_some() && board[y + *inc].get(x - *inc).is_some())
        .map(|inc| board[y + inc][x - inc])
        .find(|t| matches!(t, Tile::Occupied | Tile::Empty));
    match first_vis {
        Some(Tile::Empty) | None => 0,
        Some(Tile::Occupied) => 1,
        _ => panic!("Unexpected!"),
    }
}

type CountTileFn = fn((usize, usize), &[Vec<Tile>]) -> usize;
fn count_visible_occupied((y, x): (usize, usize), board: &[Vec<Tile>]) -> usize {
    let counters: [CountTileFn; 8] = [
        count_north,
        count_north_east,
        count_east,
        count_south_east,
        count_south,
        count_south_west,
        count_west,
        count_north_west,
    ];
    counters.iter().map(|counter| counter((y, x), &board)).sum()
}

pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("inputs/day11.txt").unwrap());
    println!("{}", part_1(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("inputs/day11.txt").unwrap());
    println!("{}", part_2(input));
    Ok(())
}

fn part_1(mut board: Vec<Vec<Tile>>) -> usize {
    loop {
        let new_board = iterate_board(&board, &new_tile_part_1);
        if new_board == board {
            return count_occupied(&board);
        } else {
            board = new_board;
        }
    }
}

fn part_2(mut board: Vec<Vec<Tile>>) -> usize {
    loop {
        let new_board = iterate_board(&board, &new_tile_part_2);
        if new_board == board {
            return count_occupied(&board);
        } else {
            board = new_board;
        }
    }
}

fn count_occupied(board: &[Vec<Tile>]) -> usize {
    let mut acc = 0;
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if let Tile::Occupied = board[y][x] {
                acc += 1
            }
        }
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print_count(board: Vec<Vec<usize>>) {
        for row in board.iter() {
            println!();
            for item in row.iter() {
                print!("{}", item);
            }
        }
    }

    fn print_board(board: &[Vec<Tile>]) {
        for row in board.iter() {
            println!();
            for item in row.iter() {
                match item {
                    Tile::Empty => print!("L"),
                    Tile::Occupied => print!("#"),
                    Tile::Floor => print!("."),
                }
            }
        }
    }

    #[test]
    fn just_count() {
        let input = parse_input(&fs::read_to_string("test_inputs/day11/2").unwrap());
        let row_length = input[0].len();
        let result = (0..input.len())
            .map(|row| {
                (0..row_length)
                    .map(|x| count_adjacent_occupied((row, x), &input))
                    .collect()
            })
            .collect();
        print_count(result);
    }

    #[test]
    fn just_iter_once() {
        let mut input = parse_input(&fs::read_to_string("test_inputs/day11/1").unwrap());
        for _ in 0..4 {
            input = iterate_board(&input, &new_tile_part_1);
            print_board(&input);
            println!();
        }
    }

    #[test]
    fn test_part_1() {
        let input = parse_input(&fs::read_to_string("test_inputs/day11/1").unwrap());
        assert_eq!(part_1(input), 37);
    }

    #[test]
    fn test_visible_chairs() {
        let input = parse_input(&fs::read_to_string("test_inputs/day11/visible1").unwrap());
        assert_eq!(count_visible_occupied((3, 3), &input), 0);
        let input = parse_input(&fs::read_to_string("test_inputs/day11/visible2").unwrap());
        assert_eq!(count_visible_occupied((1, 1), &input), 0);
        assert_eq!(count_visible_occupied((1, 3), &input), 1);
        let input = parse_input(&fs::read_to_string("test_inputs/day11/visible3").unwrap());
        assert_eq!(count_visible_occupied((4, 3), &input), 8);
    }

    #[test]
    fn test_iteration_part_2() {
        let mut input = parse_input(&fs::read_to_string("test_inputs/day11/1").unwrap());
        for _ in 0..2 {
            input = iterate_board(&input, &new_tile_part_2);
            print_board(&input);
            println!();
        }
    }

    #[test]
    fn test_part_2() {
        let input = parse_input(&fs::read_to_string("test_inputs/day11/1").unwrap());
        assert_eq!(part_2(input), 26);
    }
}
