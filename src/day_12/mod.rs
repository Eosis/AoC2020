use std::fs;

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .split('\n')
        .map(|line| {
            let mut iter = line.chars();
            let direction = iter.next().unwrap();
            let value = iter.collect::<String>().parse().unwrap();
            (direction, value)
        })
        .collect()
}

pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("./inputs/day12.txt").unwrap());
    println!("{}", part_1(&input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("./inputs/day12.txt").unwrap());
    println!("{}", part_2(&input));
    Ok(())
}

type Instruction = (char, i64);

fn next_compass_location((y, x): (i64, i64), direction: char, value: i64) -> (i64, i64) {
    match direction {
        'N' => (y - value, x),
        'E' => (y, x + value),
        'S' => (y + value, x),
        'W' => (y, x - value),
        _ => panic!("Invalid directionin the new location function"),
    }
}

fn new_location_and_state_complexy(
    (facing, (y, x)): (char, (i64, i64)),
    direction: char,
    value: i64,
) -> (char, (i64, i64)) {
    let facings = ['N', 'E', 'S', 'W'];
    match direction {
        'R' => {
            let new_facing = facings
                .iter()
                .cycle()
                .skip_while(|dir| **dir != facing)
                .nth(value as usize / 90)
                .copied()
                .unwrap();
            (new_facing, (y, x))
        }
        'L' => {
            let new_facing = facings
                .iter()
                .rev()
                .cycle()
                .skip_while(|dir| **dir != facing)
                .nth(value as usize / 90)
                .copied()
                .unwrap();
            (new_facing, (y, x))
        }
        'F' => match facing {
            'N' => (facing, (y - value, x)),
            'E' => (facing, (y, x + value)),
            'S' => (facing, (y + value, x)),
            'W' => (facing, (y, x - value)),
            _ => panic!("Wrong direction in the complexy function"),
        },
        _ => panic!("Incorrect direction in the complexy function"),
    }
}

fn part_1(input: &[Instruction]) -> usize {
    let after_simple = input
        .iter()
        .filter(|(dir, _)| matches!(dir, 'N' | 'E' | 'S' | 'W'))
        .fold((0, 0), |current, instruction| {
            next_compass_location(current, instruction.0, instruction.1)
        });
    let initial_state = ('E', (0, 0));
    let after_complexy = input
        .iter()
        .filter(|(dir, _)| matches!(dir, 'R' | 'L' | 'F'))
        .fold(initial_state, |current, instruction| {
            new_location_and_state_complexy(current, instruction.0, instruction.1)
        });
    let result = (
        after_simple.0 + after_complexy.1 .0,
        after_simple.1 + after_complexy.1 .1,
    );
    result.0.abs() as usize + result.1.abs() as usize
}

#[derive(Debug)]
struct ShipState {
    waypoint: (i64, i64),
    ship: (i64, i64),
}

fn rotate_pos((y, x): (i64, i64)) -> (i64, i64) {
    (x, -y)
}

fn rotate_neg((y, x): (i64, i64)) -> (i64, i64) {
    (-x, y)
}

fn next_rotaty_waypoint_location((y, x): (i64, i64), direction: char, value: i64) -> (i64, i64) {
    match direction {
        'R' => (0..(value / 90)).fold((y, x), |acc, _| rotate_pos(acc)),
        'L' => (0..(value / 90)).fold((y, x), |acc, _| rotate_neg(acc)),
        _ => panic!("Unexpected Coordinate Rotation in next_rotaty_waypoint_location"),
    }
}

fn move_waypoint(instruction: Instruction, state: ShipState) -> ShipState {
    match instruction.0 {
        'N' | 'E' | 'S' | 'W' => ShipState {
            waypoint: next_compass_location(state.waypoint, instruction.0, instruction.1),
            ship: state.ship,
        },
        'R' | 'L' => ShipState {
            waypoint: next_rotaty_waypoint_location(state.waypoint, instruction.0, instruction.1),
            ship: state.ship,
        },
        _ => panic!("Unknown instruction in move_waypoint"),
    }
}

fn move_ship(instruction: Instruction, state: ShipState) -> ShipState {
    let times = instruction.1;
    ShipState {
        waypoint: state.waypoint,
        ship: (
            state.ship.0 + state.waypoint.0 * times,
            state.ship.1 + state.waypoint.1 * times,
        ),
    }
}

fn new_shipstate(instruction: Instruction, state: ShipState) -> ShipState {
    match instruction.0 {
        'N' | 'E' | 'S' | 'W' | 'R' | 'L' => move_waypoint(instruction, state),
        'F' => move_ship(instruction, state),
        _ => panic!("Unknown instruction in the new_shipstate funciton"),
    }
}

fn part_2(input: &[Instruction]) -> usize {
    let state = ShipState {
        waypoint: (-1, 10),
        ship: (0, 0),
    };
    let result = input
        .iter()
        .fold(state, |state, instruction| new_shipstate(*instruction, state));
    result.ship.0.abs() as usize + result.ship.1.abs() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day12").unwrap());
        assert_eq!(part_1(&input), 25)
    }

    #[test]
    fn test_part_2() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day12").unwrap());
        assert_eq!(part_2(&input), 286)
    }
}
