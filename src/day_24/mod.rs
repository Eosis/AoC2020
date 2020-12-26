use hashbrown::HashMap;
use itertools::Itertools;
use std::fs;

pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("./inputs/day_24").unwrap());
    println!("{}", part_1(input));
    Ok(())
}

fn part_1(input: Vec<Vec<Direction>>) -> usize {
    let tiles = set_tiles_from_directions(input);
    count_black_tiles_on_floor(&tiles)
}

#[inline]
fn count_black_tiles_on_floor(floor: &HashMap<(i32, i32), Tile>) -> usize {
    floor
        .iter()
        .filter(|(_, v)| matches!(v.state, TileColor::Black))
        .count()
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("./inputs/day_24").unwrap());
    println!("{}", part_2(input, 100));
    Ok(())
}

fn part_2(input: Vec<Vec<Direction>>, iterations: usize) -> usize {
    let floor = set_tiles_from_directions(input);
    count_black_tiles_on_floor(&run_game_of_life(floor, iterations))
}

fn str_to_direction(input: &str) -> Direction {
    match input {
        "ne" => Direction::NE,
        "e" => Direction::E,
        "se" => Direction::SE,
        "sw" => Direction::SW,
        "w" => Direction::W,
        "nw" => Direction::NW,
        _ => panic!("Unknown direction passed into str_to_direction"),
    }
}

fn line_to_tile_directions(line: &str) -> Vec<Direction> {
    line.chars()
        .batching(|it| match it.next() {
            None => None,
            Some(c) => match c {
                'n' | 's' => Some([c, it.next().unwrap()].iter().join("")),
                _ => Some(c.to_string()),
            },
        })
        .map(|individual| str_to_direction(&individual))
        .collect()
}

fn parse_input(input: &str) -> Vec<Vec<Direction>> {
    input.split('\n').map(line_to_tile_directions).collect()
}

fn trundle_tile(directions: Vec<Direction>) -> (i32, i32) {
    directions.iter().fold((0, 0), |acc, direction| {
        let to_add = direction.direction_to_offset();
        (acc.0 + to_add.0, acc.1 + to_add.1)
    })
}

fn set_tiles_from_directions(directions: Vec<Vec<Direction>>) -> HashMap<(i32, i32), Tile> {
    let mut tile_locations = HashMap::new();
    for (_, tile) in directions.into_iter().enumerate() {
        let result = trundle_tile(tile);
        let mut entry = tile_locations
            .entry(result) // Get the tile to flip
            .or_insert(Tile {
                location: result,
                state: TileColor::White,
            });
        entry.state = entry.state.flip();
    }
    tile_locations
}

fn adjacent_black_tiles(tile: Tile, floor: &HashMap<(i32, i32), Tile>) -> usize {
    neighbors(tile).into_iter().fold(0, |acc, pos_to_check| {
        let neighbor_color = floor
            .get(&pos_to_check)
            .map(|tile| tile.state)
            .unwrap_or(TileColor::White);
        if neighbor_color == TileColor::Black {
            acc + 1
        } else {
            acc
        }
    })
}

fn neighbors(tile: Tile) -> Vec<(i32, i32)> {
    let adjacent_offsets = Direction::adjacent_offsets();
    let current_position = tile.location;
    adjacent_offsets
        .iter()
        .map(|offset| (current_position.0 + offset.0, current_position.1 + offset.1))
        .collect()
}

fn new_tile_state(tile: Tile, floor: &HashMap<(i32, i32), Tile>) -> TileColor {
    let adjacent_black_tiles = adjacent_black_tiles(tile, floor);
    match tile.state {
        TileColor::Black => {
            if adjacent_black_tiles == 0 || adjacent_black_tiles > 2 {
                TileColor::White
            } else {
                TileColor::Black
            }
        }
        TileColor::White => {
            if adjacent_black_tiles == 2 {
                TileColor::Black
            } else {
                TileColor::White
            }
        }
    }
}

fn print_floor(floor: &HashMap<(i32, i32), Tile>) {
    // println!("{:#?}", floor);
    let y_extent = (
        floor.iter().map(|(k, _)| k.0).min().unwrap(),
        floor.iter().map(|(k, _)| k.0).max().unwrap(),
    );
    let x_extent = (
        floor.iter().map(|(k, _)| k.1).min().unwrap(),
        floor.iter().map(|(k, _)| k.1).max().unwrap(),
    );
    print!("          ");
    let x_range_to_print = (-x_extent.1 - 1)..=(x_extent.1 + 1);
    let y_range_to_print = (-y_extent.1 - 1)..=(y_extent.1 + 1);
    for x in x_range_to_print.clone() {
        if x % 2 == 0 {
            print!("{} ", x.abs());
        }
    }

    for y in y_range_to_print {
        println!();
        print!("y: {:2}  |  ", y);
        if y.abs() % 2 == 1 {
            print!(" ");
        }
        let x_mod_to_print = y.abs() % 2;
        for x in x_range_to_print.clone() {
            if x.abs() % 2 != x_mod_to_print {
                continue;
            }
            let to_print = floor.get(&(y, x));
            let to_print = match to_print {
                None => ".",
                Some(&Tile { state: color, .. }) => match color {
                    TileColor::Black => "b",
                    TileColor::White => "W",
                },
            };
            print!("{} ", to_print);
        }
    }
    println!();
}

const DEBUG: bool = false;
fn run_game_of_life(mut floor: HashMap<(i32, i32), Tile>, iterations: usize) -> HashMap<(i32, i32), Tile> {
    if DEBUG {
        println!("Starting State");
        print_floor(&floor);
    }

    for i in 0..iterations {
        let mut new_floor: HashMap<(i32, i32), Tile> = HashMap::new();
        for (position, tile) in floor.iter().filter(|(_, v)| v.state == TileColor::Black) {
            let mut tiles_to_set = vec![*position];
            tiles_to_set.append(&mut neighbors(*tile));
            let tiles_to_set: Vec<_> = tiles_to_set
                .iter()
                .map(|tile| {
                    floor.get(tile).copied().unwrap_or(Tile {
                        location: *tile,
                        state: TileColor::White,
                    })
                })
                .collect();

            for tile in tiles_to_set {
                if new_floor.get(&(tile.location)).is_none() {
                    let new_tile = Tile {
                        location: tile.location,
                        state: new_tile_state(tile, &floor),
                    };
                    new_floor.insert(new_tile.location, new_tile);
                }
            }
        }
        floor = new_floor;
        println!("Iteration: {}", i);
        if DEBUG {
            println!("After iteration {}", i);
            print_floor(&floor);
        }
    }
    floor
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    location: (i32, i32),
    state: TileColor,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum TileColor {
    White,
    Black,
}

impl TileColor {
    fn flip(&self) -> TileColor {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
    NE,
    E,
    SE,
    SW,
    W,
    NW,
}

impl Direction {
    fn direction_to_offset(&self) -> (i32, i32) {
        match self {
            Self::NE => (1, 1),
            Self::E => (0, 2),
            Self::SE => (-1, 1),
            Self::SW => (-1, -1),
            Self::W => (0, -2),
            Self::NW => (1, -1),
        }
    }

    #[inline]
    fn adjacent_offsets() -> Vec<(i32, i32)> {
        [Self::NE, Self::E, Self::SE, Self::SW, Self::W, Self::NW]
            .iter()
            .map(Direction::direction_to_offset)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day_24").unwrap());
        assert_eq!(part_1(input), 10);
    }
    #[test]
    fn test_input_parsing() {
        let correct = vec![
            Direction::SE,
            Direction::SW,
            Direction::NE,
            Direction::SW,
            Direction::SW,
            Direction::SE,
            Direction::NW,
            Direction::W,
            Direction::NW,
            Direction::SE,
        ];
        assert_eq!(line_to_tile_directions("seswneswswsenwwnwse"), correct);
        assert_eq!(trundle_tile(correct), (-3, -3));
    }

    #[test]
    fn test_part_2() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day_24").unwrap());
        assert_eq!(part_2(input.clone(), 1), 15);
        assert_eq!(part_2(input.clone(), 3), 25);
        assert_eq!(part_2(input.clone(), 50), 566);
        assert_eq!(part_2(input, 100), 2208);
    }

    #[test]
    fn test_parts() {
        let tiles = vec![
            Tile {
                location: (0, 2),
                state: TileColor::White,
            },
            Tile {
                location: (-1, -1),
                state: TileColor::Black,
            },
            Tile {
                location: (0, 2),
                state: TileColor::White,
            },
            Tile {
                location: (1, -1),
                state: TileColor::Black,
            },
        ];
        let floor: HashMap<(i32, i32), Tile> = tiles.into_iter().map(|tile| (tile.location, tile)).collect();
        let tile_to_check = Tile {
            location: (0, 0),
            state: TileColor::White,
        };
        assert_eq!(adjacent_black_tiles(tile_to_check, &floor), 2);
        assert_eq!(new_tile_state(tile_to_check, &floor), TileColor::Black);
    }

    #[test]
    fn test_print_floor() {
        let tiles = vec![
            Tile {
                location: (0, -2),
                state: TileColor::White,
            },
            Tile {
                location: (-1, -1),
                state: TileColor::Black,
            },
            Tile {
                location: (0, 2),
                state: TileColor::White,
            },
            Tile {
                location: (1, -1),
                state: TileColor::Black,
            },
        ];
        let floor: HashMap<(i32, i32), Tile> = tiles.into_iter().map(|tile| (tile.location, tile)).collect();
        print_floor(&floor);
    }

    #[test]
    fn test_simple_game() {
        let tiles = vec![
            Tile {
                location: (-1, -1),
                state: TileColor::Black,
            },
            Tile {
                location: (-1, 1),
                state: TileColor::Black,
            },
            Tile {
                location: (0, 0),
                state: TileColor::White,
            },
        ];
        let floor = tiles.into_iter().map(|tile| (tile.location, tile)).collect();
        run_game_of_life(floor, 1);
    }
}
