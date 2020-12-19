use std::collections::VecDeque;
use std::fs;
use std::ops::Add;

#[allow(dead_code)]
pub fn solve_part_1() -> Result<(), ()> {
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("./inputs/day17.txt").unwrap());
    println!("{}", part_2(input));
    Ok(())
}

#[allow(dead_code)]
fn part_1(_: Board) -> usize {
    0
}

fn part_2(mut board: Board) -> usize {
    for _ in 0..6 {
        board = board.iterate()
    }
    board.count_total_active()
}

fn parse_input(input: &str) -> Board {
    // # z, y, x addressing
    let mut w: HyperCube = VecDeque::new();
    let mut z: Cube = VecDeque::new();
    let first_layer = input
        .split('\n')
        .map(|row| row.chars().map(|c| c == '#').collect::<VecDeque<bool>>())
        .collect::<VecDeque<VecDeque<bool>>>();
    let y_offset = (first_layer.len() / 2) as i32;
    let x_offset = (first_layer[0].len() / 2) as i32;
    z.push_front(first_layer);
    w.push_front(z);
    Board {
        cubes: w,
        x_offset,
        y_offset,
        z_offset: 0,
        w_offset: 0,
        offsets_to_check: get_points(),
    }
}

type HyperCube = VecDeque<Cube>;
type Cube = VecDeque<Layer>;
type Layer = VecDeque<Row>;
type Row = VecDeque<bool>;
#[derive(Clone, Debug)]
struct Board {
    cubes: HyperCube,
    x_offset: i32,
    y_offset: i32,
    z_offset: i32,
    w_offset: i32,
    offsets_to_check: Vec<Point>,
}

#[derive(Debug, Clone, Copy)]
struct Point(i32, i32, i32, i32);

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2, self.3 + other.3)
    }
}

impl Board {
    fn get(&self, Point(w, z, y, x): Point) -> Option<bool> {
        // println!("When getting {:?}, I'm looking for ({}, {}, {})", Point(z, y, x),  z + self.z_offset, y + self.y_offset, x + self.x_offset);
        if (w + self.w_offset) < 0 || (z + self.z_offset) < 0 || (y + self.y_offset) < 0 || (x + self.x_offset) < 0 {
            None
        } else {
            self.cubes
                .get((w + self.w_offset) as usize)
                .and_then(|cube| cube.get((z + self.z_offset) as usize))
                .and_then(|layer| layer.get((y + self.y_offset) as usize))
                .and_then(|row| row.get((x + self.x_offset) as usize).copied())
        }
    }

    fn set(&mut self, Point(w, z, y, x): Point, active: bool) {
        if w.abs() > self.w_offset {
            self.expand_w();
        }
        if z.abs() > self.z_offset {
            self.expand_z();
        }
        if y.abs() > self.y_offset {
            self.expand_y();
        }
        if x.abs() > self.x_offset {
            self.expand_x();
        }
        self.cubes[(w + self.w_offset) as usize][(z + self.z_offset) as usize][(y + self.y_offset) as usize]
            [(x + self.x_offset) as usize] = active;
    }
    fn expand_w(&mut self) {
        let z_len = self.cubes[0].len();
        let y_len = self.cubes[0][0].len();
        let x_len = self.cubes[0][0][0].len();
        let new_cube: Cube = (0..z_len)
            .map(|_| -> Layer {
                (0..y_len)
                    .map(|_| -> Row { (0..x_len).map(|_| false).collect() })
                    .collect()
            })
            .collect();
        self.cubes.push_front(new_cube.clone());
        self.cubes.push_back(new_cube);
        self.w_offset += 1
    }

    fn expand_z(&mut self) {
        let y_len = self.cubes[0][0].len();
        let x_len = self.cubes[0][0][0].len();
        let new_layer: Layer = (0..y_len)
            .map(|_| -> Row { (0..x_len).map(|_| false).collect() })
            .collect();
        for cube in self.cubes.iter_mut() {
            cube.push_front(new_layer.clone());
            cube.push_back(new_layer.clone());
        }
        self.z_offset += 1
    }

    fn expand_y(&mut self) {
        let x_len = self.cubes[0][0][0].len();
        let new_row: Row = (0..x_len).map(|_| false).collect();
        for cube in self.cubes.iter_mut() {
            for layer in cube.iter_mut() {
                layer.push_front(new_row.clone());
                layer.push_back(new_row.clone());
            }
        }
        self.y_offset += 1
    }

    fn expand_x(&mut self) {
        for cube in self.cubes.iter_mut() {
            for layer in cube.iter_mut() {
                for row in layer.iter_mut() {
                    row.push_front(false);
                    row.push_back(false);
                }
            }
        }
        self.x_offset += 1
    }

    fn count_active_neighbours(&self, point: Point) -> usize {
        let result = self
            .offsets_to_check
            .iter()
            .map(|offset| *offset + point)
            .map(|neighbouring_point| self.get(neighbouring_point).unwrap_or(false))
            .filter(|active| *active)
            .count();
        result
    }

    // If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
    // If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
    fn apply_rules(&self, point: Point) -> bool {
        let active = self.get(point).unwrap_or(false);
        let neighbour_count = self.count_active_neighbours(point);
        if active {
            (2..=3).contains(&neighbour_count)
        } else {
            neighbour_count == 3
        }
    }

    fn iterate(self) -> Self {
        let mut new_board = self.clone(); // Expensive copy!
        for w in (-(self.w_offset + 1))..=(self.w_offset + 1) {
            for z in (-(self.z_offset + 1))..=(self.z_offset + 1) {
                for y in (-(self.y_offset + 1))..=(self.y_offset + 1) {
                    for x in (-(self.x_offset + 1))..=(self.x_offset + 1) {
                        // println!("Checking {:?}", (z, y, x));
                        let new_active_state = self.apply_rules((w, z, y, x).into());
                        new_board.set((w, z, y, x).into(), new_active_state);
                    }
                }
            }
        }
        new_board
    }

    fn count_total_active(&self) -> usize {
        self.cubes
            .iter()
            .flat_map(|cube| {
                cube.iter()
                    .flat_map(|layer| layer.iter().flat_map(|row| row.iter().copied()))
            })
            .filter(|x| *x)
            .count()
    }

    // fn print_layers(&self) {
    //     for (z, layer) in self.cubes.iter().enumerate() {
    //         Board::print_layer(layer, z as i32 - self.z_offset);
    //     }
    // }
    #[allow(dead_code)]
    fn print_layer(layer: &Layer, z: i32) {
        println!("z = {}", z);
        for row in layer.iter() {
            for cube in row.iter() {
                if *cube {
                    print!("#")
                } else {
                    print! {"."}
                }
            }
            println!();
        }
        println!();
    }
}

fn get_points() -> Vec<Point> {
    // % 3 == 0 => -1, == % 3 - 1
    // % 3 == 1 => 0,
    // % 3 == 2 => 1,
    (0..3i32.pow(4u32))
        .map(|i| (((i / 27) % 3) - 1, ((i / 9) % 3) - 1, ((i / 3) % 3) - 1, i % 3 - 1))
        .filter(|x| *x != (0, 0, 0, 0))
        .map(|x| x.into())
        .collect()
}

impl From<(i32, i32, i32, i32)> for Point {
    fn from((w, z, y, x): (i32, i32, i32, i32)) -> Self {
        Point(w, z, y, x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let board = parse_input(&fs::read_to_string("./test_inputs/day17").unwrap());
        assert_eq!(part_1(board), 848);
    }

    #[test]
    fn test_parsing_board() {
        let board = parse_input(&fs::read_to_string("./test_inputs/day17").unwrap());
        assert_eq!(board.get((0, 0, 0, 0).into()), Some(false));
        assert_eq!(board.get((0, 1, 0, 0).into()), None);
        assert_eq!(board.get((0, 0, -1, 0).into()), Some(true));
    }

    #[test]
    fn test_parsing_board_and_adding_layers_and_rows() {
        let mut board = parse_input(&fs::read_to_string("./test_inputs/day17").unwrap());
        assert_eq!(board.get((0, 0, 0, 0).into()), Some(false));
        assert_eq!(board.get((0, 1, 0, 0).into()), None);
        assert_eq!(board.get((0, 0, -1, 0).into()), Some(true));
        board.expand_x();
        board.expand_y();
        board.expand_z();
        assert_eq!(board.get((0, 0, 0, 0).into()), Some(false));
        assert_eq!(board.get((0, 1, 0, 0).into()), Some(false));
        assert_eq!(board.get((0, 0, -1, 0).into()), Some(true));
        assert_eq!(board.get((0, -1, 0, 0).into()), Some(false));
        assert_eq!(board.get((0, -2, 0, 0).into()), None);
        assert_eq!(board.get((0, 0, -3, 0).into()), None);
        assert_eq!(board.get((0, 0, -2, 0).into()), Some(false));
        assert_eq!(board.get((0, 0, -2, -10).into()), None);
        assert_eq!(board.get((0, 0, 0, -2).into()), Some(false));
    }

    #[test]
    fn test_iterating_board() {
        let mut board = parse_input(&fs::read_to_string("./test_inputs/day17").unwrap());
        for _ in 0..6 {
            board = board.iterate();
        }
    }

    #[test]
    fn test_count_active() {
        let board = parse_input(&fs::read_to_string("./test_inputs/day17").unwrap());
        assert_eq!(board.count_total_active(), 5);
    }
}
