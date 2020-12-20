use hashbrown::HashMap;
use std::fmt::Display;
use std::{fmt, fs};
use itertools::Itertools;

pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("./inputs/day20.txt").unwrap());
    println!("{}", part_1(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("./inputs/day20.txt").unwrap());
    println!("{:?}", part_2(input, 12));
    Ok(())
}

fn part_2(input: Problem, grid_size: usize) -> usize {
    let map_fragments = input.map_fragments.clone();
    let solved_grid = solve_complete_grid(input, grid_size).unwrap();
    let resulting_pieces: HashMap<(usize, usize), Tile> = solved_grid
        .into_iter()
        .enumerate()
        .flat_map( |(y, row)| {
            row.into_iter().enumerate().map(move |(x, tile)| ((y, x), tile.unwrap()))
        })
        .collect();
    let resulting_map = build_map_from_original(resulting_pieces, map_fragments, grid_size);
    let big_fragment = MapFragment {
        id: 1000,
        fragment: resulting_map,
    };
    let (rotation, flip) = find_rotation_for_nessies(&big_fragment);
    let nessie_count = count_nessies_in_map(&big_fragment.rotated(rotation).flipped(flip));
    let choppy_count = big_fragment.fragment.iter().flat_map(|row| row.iter())
        .filter(|&&c| c == '#').count();
    choppy_count - (nessie_count * 15)
}

fn build_map_from_original(resulting_pieces: HashMap<(usize, usize), Tile>,
                           mut map_fragments: HashMap<u32, MapFragment>,
                           grid_size: usize) -> Vec<Vec<char>> {
    let mut full_corrected_map: Vec<Vec<char>> = vec![vec![]; grid_size * 8]; // There will be 8 lines for each part.
    for ((y, _), tile) in resulting_pieces.iter().sorted_by_key(|(k, _)| *k) {
        let map_fragment = map_fragments.remove(&tile.id).unwrap();
        let mut corrected_map_fragment = map_fragment.rotated(tile.rotated).flipped(tile.flipped);
        for i in 0..8 {
            full_corrected_map[y*8 + i].append(&mut corrected_map_fragment.fragment[i]);
        }
    }
    full_corrected_map
}

type Grid = Vec<Vec<Option<Tile>>>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Tile {
    id: u32,
    top: u32,
    right: u32,
    bottom: u32,
    left: u32,
    rotated: usize,
    flipped: Flip,
}

impl Tile {
    fn rotated(&self, rotation: usize) -> Tile {
        (0..rotation).fold(self.clone(), |acc, _| acc.rotate())
    }

    fn rotate(&self) -> Tile {
        Tile {
            id: self.id,
            right: self.top,
            bottom: Tile::reversed(self.right),
            left: self.bottom,
            top: Tile::reversed(self.left),
            rotated: (self.rotated + 1) % 4,
            flipped: self.flipped,
        }
    }

    fn flipped(&self, flip: Flip) -> Tile {
        match flip {
            Flip::Zero => self.clone(),
            Flip::Y => Tile {
                id: self.id,
                top: self.bottom,
                bottom: self.top,
                right: Tile::reversed(self.right),
                left: Tile::reversed(self.left),
                flipped: Flip::Y,
                rotated: self.rotated,
            },
            Flip::X => Tile {
                id: self.id,
                right: self.left,
                left: self.right,
                top: Tile::reversed(self.top),
                bottom: Tile::reversed(self.bottom),
                flipped: Flip::X,
                rotated: self.rotated
            },
        }
    }

    fn reversed(value: u32) -> u32 {
        let mut new = 0;
        for pos in 0..10 {
            let old_bit = (value & (1 << pos)) >> pos;
            let new_pos = 9 - pos;
            new |= old_bit << new_pos;
        }
        new
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "id: {},\n\
            top: {:010b},\n\
            right: {:010b},\n\
            bottom: {:010b},\n\
            left: {:010b},\n",
            self.id, self.top, self.right, self.bottom, self.left
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Flip {
    Zero,
    Y,
    X,
}

struct Problem {
    tiles: HashMap<u32, Tile>,
    map_fragments: HashMap<u32, MapFragment>,
}

fn solve_complete_grid(input: Problem, grid_length: usize) -> Option<Grid> {
    let grid = vec![vec![None; grid_length]; grid_length];
    let (corners, edges) = determine_border_tiles(&input.tiles);
    let mut tiles = input.tiles;
    for (k, _) in corners.iter().chain(edges.iter()) {
        tiles.remove(k);
    }
    let grid_with_border = solve_border(corners, edges, grid);
    if let None = grid_with_border {
        println!("We couldn't even determine the border... so shameful...");
        return None;
    }
    solve_grid(tiles, grid_with_border.unwrap())
}

fn part_1(input: Problem) -> u64 {
    let (corners, _) = determine_border_tiles(&input.tiles);
    corners.iter().map(|(_, t)| t.id as u64).product::<u64>()
}

fn hash_dots_to_int(hash_dots: &str) -> u32 {
    let as_bin: String = hash_dots.chars().map(|c| if c == '#' { '1' } else { '0' }).collect();
    u32::from_str_radix(&as_bin, 2).unwrap()
}

fn get_id_from_tile_description(input: &str) -> u32 {
    input
        .split('\n')
        .next()
        .and_then(|line| line.split(&[' ', ':'][..]).nth(1).unwrap().parse::<u32>().ok())
        .unwrap()
}

fn tile_from_tile_description(input: &str) -> Tile {
    let id = get_id_from_tile_description(input);
    let tile_iter = input.split('\n').skip(1);
    let top = tile_iter.clone().nth(0).map(hash_dots_to_int).unwrap();
    let bottom = tile_iter.clone().last().map(hash_dots_to_int).unwrap();
    let right: String = tile_iter.clone().map(|line| line.chars().last().unwrap()).collect();
    let right = hash_dots_to_int(&right);
    let left: String = tile_iter.clone().map(|line| line.chars().next().unwrap()).collect();
    let left = hash_dots_to_int(&left);
    Tile {
        id,
        top,
        right,
        bottom,
        left,
        flipped: Flip::Zero,
        rotated: 0,
    }
}

#[derive(Clone, Eq, PartialEq)]
struct MapFragment {
    id: u32,
    fragment: Vec<Vec<char>>
}

fn rotate_vec_of_ch(vec_of_ch: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let y_size = vec_of_ch.len();
    let x_size = vec_of_ch[0].len();
    let mut new = vec![vec!['0'; x_size]; y_size];
    for (y, row) in vec_of_ch.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            new[x][y_size - 1 - y] = *c;
        }
    }
    new
}

fn flip_vec_of_ch(vec_of_ch: Vec<Vec<char>>, flip: Flip) -> Vec<Vec<char>> {
    match flip {
        Flip::Zero => vec_of_ch,
        Flip::Y => {
            let mut new = vec![];
            for i in (0..vec_of_ch.len()).rev() {
                new.push(vec_of_ch[i].clone())
            }
            new
        },
        Flip::X => {
            let y_size = vec_of_ch.len();
            let x_size = vec_of_ch[0].len();
            let mut new = vec![vec!['0'; x_size]; y_size];
            for y in 0..y_size {
                for (i, x) in (0..x_size).rev().enumerate() {
                    new[y][i] = vec_of_ch[y][x];
                }
            }
            new
        },
    }
}

impl MapFragment {
    fn rotate(&self) -> MapFragment {
        let new = rotate_vec_of_ch(self.fragment.clone());
        MapFragment {
            id: self.id,
            fragment: new
        }
    }

    fn rotated(&self, rotation: usize) -> MapFragment {
        (0..rotation).fold(self.clone(), |acc, _| acc.rotate())
    }

    fn flipped(&self, flip: Flip) -> MapFragment {
        let new = flip_vec_of_ch(self.fragment.clone(), flip);
        MapFragment {
            id: self.id,
            fragment: new,
        }
    }
}

fn fragment_from_tile_description(desc: &str) -> MapFragment {
    let id = get_id_from_tile_description(desc);
    let fragment = desc.split('\n')
        .skip(2)
        .take(8)
        .map(|row| row.chars().skip(1).take(8).collect())
        .collect();
    MapFragment {
        id,
        fragment
    }
}

fn parse_input(input: &str) -> Problem {
    let tiles = input
        .split("\n\n")
        .map(|tile_desc| tile_from_tile_description(tile_desc))
        .map(|tile| (tile.id, tile))
        .collect();
    let map_fragments = input.split("\n\n")
        .map(|tile_desc| fragment_from_tile_description(tile_desc))
        .map(|map_fragment| (map_fragment.id, map_fragment))
        .collect();
    Problem {
        tiles,
        map_fragments,
    }
}

fn next_pos_to_check(grid: &Grid) -> (usize, usize) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x].is_none() {
                return (y, x);
            }
        }
    }
    panic!("Oh Dear, I have run out of places to check, mebs some stupid error");
}

fn next_pos_to_check_in_border(tiles_left: usize, y_size: usize, x_size: usize) -> (usize, usize) {
    let total_border_size = y_size * 2 + (y_size - 2) * 2;
    let number_of_tile_to_place = total_border_size - tiles_left + 1;
    if number_of_tile_to_place <= x_size {
        (0, number_of_tile_to_place - 1)
    } else if number_of_tile_to_place <= (x_size + (y_size - 2 ) * 2) {
        let y = (number_of_tile_to_place - x_size - 1) / 2 + 1;
        let x = if (number_of_tile_to_place - x_size) % 2 == 0 {
            x_size - 1
        } else {
            0
        };
        (y, x)
    } else {
        let y = y_size - 1;
        let x = number_of_tile_to_place - (x_size + (y_size - 2) * 2) - 1;
        (y, x)
    }
}

// for each remaining tile
// for each rotation
//for each flipation
// insert the tile into the next available position in the grid
// if valid, solve the remaining grid without this tile:
// take tile from hashmap and clone it.
// add this tile to the grid.
// solve for the new grid and tile set.
// return the grid if it is solved from this call, otherwise loopy loopy
// if invalid,
// try the next one!
// None

fn solve_grid(tiles: HashMap<u32, Tile>, grid: Grid) -> Option<Grid> {
    // if there are no tiles left! return the solved grid. :)
    if tiles.is_empty() {
        return Some(grid);
    }
    // set the next tile position to investigate
    let (y, x) = next_pos_to_check(&grid);
    if y == x {
        println!("I've moved onto ({}, {})", y, x);
    }
    for (_, tile) in tiles.iter().sorted_by_key(|(k, _)| *k) {
        for rotation in 0..4 {
            for flip in [Flip::Zero, Flip::Y, Flip::X].iter() {
                let tile = tile.rotated(rotation).flipped(*flip);
                let tile_id = tile.id;
                let mut new_grid = grid.clone();
                new_grid[y][x] = Some(tile);
                if check_valid(&new_grid) {
                    let mut new_tiles = tiles.clone();
                    new_tiles.remove(&tile_id);
                    if let Some(grid) = solve_grid(new_tiles, new_grid) {
                        return Some(grid);
                    }
                }
            }
        }
    }
    None
}

fn is_corner_position((y, x): (usize, usize), grid: &Grid) -> bool {
    let max_y_idx = grid.len() - 1;
    let max_x_idx = grid[0].len() - 1;
    (y, x) == (0, 0) || (y, x) == (0, max_x_idx) || (y, x) == (max_y_idx, 0) || (y, x) == (max_y_idx, max_x_idx)
}

fn solve_border(corners: HashMap<u32, Tile>, edges: HashMap<u32, Tile>, grid: Grid) -> Option<Grid> {
    // if there are no tiles left! return the solved border. :)
    if corners.is_empty() && edges.is_empty() {
        return Some(grid);
    }

    // set the next tile position to investigate
    let (y, x) = next_pos_to_check_in_border(corners.len() + edges.len(), grid.len(), grid[0].len());
    let is_corner = is_corner_position((y, x), &grid);
    let to_choose_from = if is_corner {
        &corners
    } else {
        &edges
    };

    for (_, tile) in to_choose_from.clone().iter().sorted_by_key(|(k, _)| *k) {
        for rotation in 0..4 {
            for flip in [Flip::Zero, Flip::Y, Flip::X].iter() {
                let tile = tile.rotated(rotation).flipped(*flip);
                let tile_id = tile.id;
                let mut new_grid = grid.clone();
                new_grid[y][x] = Some(tile);
                if check_valid(&new_grid) {
                    let mut chosen_tile_set = (*to_choose_from).clone();
                    chosen_tile_set.remove(&tile_id);
                    let (corners, edges) = {
                        if is_corner {
                            (chosen_tile_set, edges.clone())
                        } else {
                            (corners.clone(), chosen_tile_set)
                        }
                    };
                    if let Some(grid) = solve_border(corners, edges, new_grid) {
                        return Some(grid);
                    }
                }
            }
        }
    }
    None
}

fn check_valid(grid: &Grid) -> bool {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            // println!("Checking ({}, {})", y, x);
            if grid[y][x].is_some() {
                // println!("Twas SOME");
                if !check_tile((y, x), grid) {
                    return false;
                }
            }
        }
    }
    true
}

fn check_tile((y, x): (usize, usize), grid: &Grid) -> bool {
    let max_x_idx = grid[0].len() - 1;
    let max_y_idx = grid.len() - 1;

    // check_above if present
    if y > 0 && grid[y - 1][x].is_some() {
        if !check_above(grid[y][x].as_ref().unwrap(), grid[y - 1][x].as_ref().unwrap()) {
            return false;
        }
    }

    // check_right if present
    if x < max_x_idx && grid[y][x + 1].is_some() {
        if !check_right(grid[y][x].as_ref().unwrap(), grid[y][x + 1].as_ref().unwrap()) {
            return false;
        }
    }

    // check_below if present
    if y < max_y_idx && grid[y + 1][x].is_some() {
        if !check_below(grid[y][x].as_ref().unwrap(), grid[y + 1][x].as_ref().unwrap()) {
            return false;
        }
    }

    // check_left if present
    if x > 0 && grid[y][x - 1].is_some() {
        if !check_left(grid[y][x].as_ref().unwrap(), grid[y][x - 1].as_ref().unwrap()) {
            return false;
        }
    }

    true
}

fn check_above(to_check: &Tile, above: &Tile) -> bool {
    to_check.top == above.bottom
}

fn check_right(to_check: &Tile, right: &Tile) -> bool {
    to_check.right == right.left
}

fn check_below(to_check: &Tile, below: &Tile) -> bool {
    to_check.bottom == below.top
}

fn check_left(to_check: &Tile, left: &Tile) -> bool {
    to_check.left == left.right
}

fn populate_counts_of_sides(tiles: &HashMap<u32, Tile>) -> HashMap<u32, usize> {
    tiles.iter()
        .fold(HashMap::new(), |mut acc, (_, v)| -> HashMap<u32, usize> {
            for side in [v.top, v.right, v.bottom, v.left].iter() {
                let count = acc.entry(*side).or_insert(0);
                *count += 1;
                let count = acc.entry(Tile::reversed(*side)).or_insert(0);
                *count += 1;
            }
            acc
        })
}

fn check_any_side_matches(tile: &Tile, value: u32) -> bool {
    tile.top == value || tile.bottom == value || tile.right == value || tile.left == value
}

fn count_number_of_matches(tile: &Tile, values: &[u32]) -> usize {
    values.iter().filter(|val| check_any_side_matches(tile, **val)).count()
}

fn determine_border_tiles(tiles: &HashMap<u32, Tile>) -> (HashMap<u32, Tile>, HashMap<u32, Tile>) {
    let counts_of_sides = populate_counts_of_sides(tiles);
    let single_count_values: Vec<u32> = counts_of_sides
        .iter()
        .filter(|(_, v)| **v == 1)
        .map(|(k, _)| k)
        .copied()
        .collect();
    // Corner pieces have two values that only appear once on them.
    let corners = tiles.iter()
        .filter(|(_, tile)| count_number_of_matches(*tile, &single_count_values) == 2)
        .map(|(k, t)| (*k, t.clone()))
        .sorted_by_key(|(k, _)| *k)
        .collect();

    // Edges only have one side that appears only once.
    let edges = tiles.iter()
        .filter(|(_, tile)| count_number_of_matches(*tile, &single_count_values) == 1)
        .map(|(k, t)| (*k, t.clone()))
        .sorted_by_key(|(k, _)| *k)
        .collect();

    (corners, edges)
}

const NESSIE_OFFSETS: [(usize, usize); 15] = [
    (0, 18),
    (1, 0),
    (1, 5),
    (1, 6),
    (1, 11),
    (1, 12),
    (1, 17),
    (1, 18),
    (1, 19),
    (2, 1),
    (2, 4),
    (2, 7),
    (2, 10),
    (2, 13),
    (2, 16)
];

fn scan_map_for_nessies(map: MapFragment) -> bool {
    for y in 0..=(map.fragment.len() - 3) {
        for x in 0..=(map.fragment[0].len() - 20) {
            if NESSIE_OFFSETS.iter().all(|(y_off, x_off)| map.fragment[y + *y_off][x + *x_off] == '#') {
                return true;
            }
        }
    }
    false
}

fn find_rotation_for_nessies(map: &MapFragment) -> (usize, Flip) {
    for rotation in 0..4 {
        for flip in [Flip::Zero, Flip::Y, Flip::X].iter() {
            let transformed = map.rotated(rotation).flipped(*flip);
            if scan_map_for_nessies(transformed) {
                return (rotation, *flip);
            }
        }
    }
    panic!("No nessies anywhere to be found");
}

fn count_nessies_in_map(map: &MapFragment) -> usize {
    let mut count = 0;
    for y in 0..=(map.fragment.len() - 3) {
        for x in 0..=(map.fragment[0].len() - 20) {
            if NESSIE_OFFSETS.iter().all(|(y_off, x_off)| map.fragment[y + *y_off][x + *x_off] == '#') {
                count += 1;
            }
        }
    }
    count
}
#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    #[ignore]
    fn test_input_parsing() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day20").unwrap());
        for (k, v) in input.tiles.iter().sorted_by_key(|(k, _)| **k) {
            println!("{}", k);
            println!("{}", v);
        }
    }

    #[test]
    fn test_rotated_and_flipped() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day20").unwrap());
        let tile = input.tiles.get(&1171).unwrap();
        let rotated = tile.rotated(0);
        assert_eq!(*tile, rotated);
        let rotated = tile.rotated(1);
        let expected = Tile {
            id: 1171,
            top: Tile::reversed(tile.left),
            right: tile.top,
            bottom: Tile::reversed(tile.right),
            left: tile.bottom,
            rotated: 1,
            flipped: Flip::Zero,
        };
        assert_eq!(expected, rotated);

        let rotated = tile.rotated(3);
        let expected = Tile {
            id: 1171,
            left: Tile::reversed(tile.top),
            bottom: tile.left,
            top: tile.right,
            right: Tile::reversed(tile.bottom),
            rotated: 3,
            flipped: Flip::Zero
        };
        assert_eq!(expected, rotated);

        let rotated_and_flipped = tile.rotated(1).flipped(Flip::Y);
        let expected = Tile {
            id: 1171,
            left: Tile::reversed(tile.bottom),
            bottom: Tile::reversed(tile.left),
            top: Tile::reversed(tile.right),
            right: Tile::reversed(tile.top),
            rotated: 1,
            flipped: Flip::Y
        };
        assert_eq!(expected, rotated_and_flipped);
    }

    #[test]
    fn test_bit_flippin() {
        assert_eq!(0b1000000000, Tile::reversed(0b0000000001));
        assert_eq!(0b1111100000, Tile::reversed(0b0000011111));
        assert_eq!(0b1110001110, Tile::reversed(0b0111000111));
        assert_eq!(0b0000000001, Tile::reversed(0b1000000000));
    }

    #[test]
    fn test_check_valid() {
        // The valid board here.
        // 1951    2311    3079
        // 2729    1427    2473
        // 2971    1489    1171

        let input = parse_input(&fs::read_to_string("./test_inputs/day20").unwrap());
        let tiles = input.tiles;
        let mut correct_board: Grid = vec![];
        assert!(check_valid(&correct_board));
        correct_board.push(vec![
            tiles.get(&1951).map(|t| t.flipped(Flip::Y)),
            tiles.get(&2311).map(|t| t.flipped(Flip::Y)),
            tiles.get(&3079).cloned(),
        ]);
        assert!(check_valid(&correct_board));
        correct_board.push(vec![
            tiles.get(&2729).map(|t| t.flipped(Flip::Y)),
            tiles.get(&1427).map(|t| t.flipped(Flip::Y)),
            tiles.get(&2473).map(|t| t.rotated(3).flipped(Flip::X)),
        ]);
        assert!(check_valid(&correct_board));
        correct_board.push(vec![
            tiles.get(&2971).map(|t| t.flipped(Flip::Y)),
            tiles.get(&1489).map(|t| t.flipped(Flip::Y)),
            tiles.get(&1171).map(|t| t.flipped(Flip::X)),
        ]);
        assert!(check_valid(&correct_board))
    }

    #[test]
    fn test_solve_complete_grid() {
        // I'm a little unsure that this test is correct... I sort of fudged it in the end.
        let input = parse_input(&fs::read_to_string("./test_inputs/day20").unwrap());
        let result = solve_complete_grid(input, 3).unwrap();
        let result: HashMap<u32, Tile> = result
            .into_iter()
            .flat_map(|row| row.into_iter())
            .inspect(|item| println!("{}", item.as_ref().unwrap().id))
            .map(|tile| (tile.as_ref().unwrap().id, tile.unwrap()))
            .collect();
        assert_eq!((result.get(&3079).unwrap().rotated, result.get(&3079).unwrap().flipped), (2, Flip::Zero));
        assert_eq!((result.get(&2473).unwrap().rotated, result.get(&2473).unwrap().flipped), (1, Flip::X));
        assert_eq!((result.get(&2971).unwrap().rotated, result.get(&2971).unwrap().flipped), (0, Flip::X));
    }

    #[test]
    #[ignore]
    fn count_sides_in_input() {
        let input = parse_input(&fs::read_to_string("./inputs/day20.txt").unwrap());
        let counts_of_sides = populate_counts_of_sides(&input.tiles);
        for (k, v) in counts_of_sides.iter().sorted_by_key(|(_, v)| **v) {
            println!("{} appears {} times", k, v);
        }
        println!("There are {} sides with only 1 entry", counts_of_sides.iter().filter(|(_, v)| **v == 1).count());
    }

    #[test]
    fn test_part_1() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day20").unwrap());
        assert_eq!(part_1(input), 20899048083289);
    }

    #[test]
    #[ignore]
    fn test_determining_border_pieces() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day20").unwrap());
        let (corners, edges) = determine_border_tiles(&input.tiles);
        assert_eq!(corners.len() + edges.len(), 8);
    }

    #[test]
    fn test_next_pos_to_check_in_border() {
        for i in 0..12 {
            println!("{:?}", next_pos_to_check_in_border(12 - i, 4, 4));
        }
    }

    #[test]
    fn test_solve_border() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day20").unwrap());
        let mut grid = vec![];
        grid.push(vec![None, None, None]);
        grid.push(vec![None, None, None]);
        grid.push(vec![None, None, None]);
        let (corners, edges) = determine_border_tiles(&input.tiles);
        let result = solve_border(corners, edges, grid);
        assert!(result.is_some());
        let grid = result.unwrap();
        let corners = [
            grid[0][0].clone().unwrap().id,
            grid[0][2].clone().unwrap().id,
            grid[2][0].clone().unwrap().id,
            grid[2][2].clone().unwrap().id,
        ];
        assert_eq!(corners.iter().map(|id| *id as u64).product::<u64>(), 20899048083289);
    }

    #[test]
    fn test_getting_map_fragments() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day20").unwrap());
        let result = input.map_fragments.get(&2729).unwrap();
        let full: String = result.fragment.iter().map(|row| row.iter().collect::<String>()).join("\n");
        let expected = "###.#...\n\
                              .#.#....\n\
                              ...#..#.\n\
                              ##..##.#\n\
                              #.####..\n\
                              ###.#.#.\n\
                              #.####..\n\
                              #..#.##.";
        assert_eq!(full, expected);
    }

    #[test]
    fn test_printing_final_map() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day20").unwrap());
        let resulting_map = part_2(input, 3);
        let big_fragment = MapFragment {
            id: 1000,
            fragment: resulting_map,
        };

        for rotation in 0..4 {
            for flip in [Flip::Zero, Flip::Y, Flip::X].iter() {
                let new_map = big_fragment.rotated(rotation).flipped(*flip);
                println!("Printing out the map for rotation: {} and flip: {:?}", rotation, *flip);
                for row in new_map.fragment {
                    for c in row {
                        print!("{}", c);
                    }
                    println!();
                }
            }
        }
    }

    #[test]
    #[ignore]
    fn test_finding_nessies() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day20").unwrap());
        let resulting_map = part_2(input, 3);
        let big_fragment = MapFragment {
            id: 1000,
            fragment: resulting_map,
        };

        let (rotation, flip) = find_rotation_for_nessies(&big_fragment);
        assert_eq!((rotation, flip), (1, Flip::Y));
    }

    #[test]
    fn determine_choppiness() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day20").unwrap());
        assert_eq!(part_2(input, 3), 273);
    }
}
