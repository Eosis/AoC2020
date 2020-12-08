use bit_vec::BitVec;
use std::convert::TryInto;
use std::fs;

#[derive(Debug, Copy, Clone)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    op: Operation,
    number: i32,
}

fn line_to_instruction(line: &str) -> Instruction {
    let mut iter = line.split_whitespace();
    let op = match iter.next().unwrap() {
        "nop" => Operation::Nop,
        "acc" => Operation::Acc,
        "jmp" => Operation::Jmp,
        _ => panic!("Unrecognized instruction"),
    };
    let number = iter.next().unwrap().parse().unwrap();
    Instruction { op, number }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.split('\n').map(line_to_instruction).collect()
}

pub fn solve_part_1() -> Result<(), ()> {
    let prog = parse_input(&fs::read_to_string("/home/rupert/code/AoC2020/inputs/day8.txt").unwrap());
    println!("{}", run_part_1(&prog));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let mut prog = parse_input(&fs::read_to_string("/home/rupert/code/AoC2020/inputs/day8.txt").unwrap());
    println!("{}", run_part_2(&mut prog));
    Ok(())
}

fn run_part_1(program: &[Instruction]) -> i32 {
    let mut visited = BitVec::from_elem(program.len(), false);
    let mut pc: i32 = 0;
    let mut acc = 0;
    while !visited.get(pc.try_into().unwrap()).unwrap() {
        visited.set(pc.try_into().unwrap(), true);
        execute_instruction(program, &mut pc, &mut acc);
    }
    acc
}

#[inline]
fn swappable(instruction: Instruction) -> bool {
    matches!(instruction.op, Operation::Nop | Operation::Jmp)
}

fn swap_instruction(instruction: Instruction) -> Instruction {
    let op = match instruction.op {
        Operation::Nop => Operation::Jmp,
        Operation::Jmp => Operation::Nop,
        _ => panic!("Given unswappable instruction"),
    };
    Instruction {
        op,
        number: instruction.number,
    }
}

fn execute_instruction(program: &[Instruction], pc: &mut i32, acc: &mut i32) {
    match program[*pc as usize] {
        Instruction { op: Operation::Nop, .. } => *pc += 1,
        Instruction {
            op: Operation::Jmp,
            number,
        } => *pc += number,
        Instruction {
            op: Operation::Acc,
            number,
        } => {
            *pc += 1;
            *acc += number;
        }
    }
}

fn test_execution(program: &[Instruction]) -> Option<i32> {
    let mut visited = BitVec::from_elem(program.len(), false);
    let mut pc: i32 = 0;
    let mut acc = 0;
    while !visited.get(pc.try_into().unwrap()).unwrap() {
        visited.set(pc.try_into().unwrap(), true);
        execute_instruction(program, &mut pc, &mut acc);
        if pc == program.len() as i32 {
            return Some(acc);
        }
    }
    None
}

fn run_part_2(program: &mut [Instruction]) -> i32 {
    if let Some(ans) = test_execution(&program) {
        return ans;
    }
    let swappable_idxs: Vec<_> = program
        .iter()
        .enumerate()
        .filter(|(_, instr)| swappable(**instr))
        .map(|(i, _)| i)
        .collect();
    for i in swappable_idxs {
        program[i] = swap_instruction(program[i]);
        if let Some(ans) = test_execution(&program) {
            return ans;
        } else {
            program[i] = swap_instruction(program[i]);
        }
    }
    panic!("Didn't find any swap that worked. So Sad!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_prog() {
        let prog = parse_input(&fs::read_to_string("/home/rupert/code/AoC2020/test_inputs/day8").unwrap());
        assert_eq!(run_part_1(&prog), 5);
    }

    #[test]
    fn test_part_two() {
        let mut prog = parse_input(&fs::read_to_string("/home/rupert/code/AoC2020/test_inputs/day8").unwrap());
        assert_eq!(run_part_2(&mut prog), 8);
    }
}
