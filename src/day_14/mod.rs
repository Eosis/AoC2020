use std::fs;
use anyhow::Result;
use regex::internal::Inst;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Mask(String),
    MemorySet(usize, u64),
}

fn line_to_instruction(line: &str) -> Instruction {
    match &line[0..2] {
        "ma" => Instruction::Mask(String::from(line.split_whitespace().last().unwrap())),
        "me" => Instruction::MemorySet(line.split(|c| c == '[' || c == ']').nth(1).unwrap().parse().unwrap(), line.split_whitespace().last().unwrap().parse().unwrap()),
        _ => panic!("Malformed input"),
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.split('\n')
        .map(line_to_instruction)
        .collect()
}

pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("./inputs/day14.txt").unwrap());
    println!( "{}", part_1(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("./inputs/day14.txt").unwrap());
    println!( "{}", part_2(input));
    Ok(())
}

fn part_1(input: Vec<Instruction>) -> u64 {
    let computer = Computer { current_mask: "X".to_string(), memory_map: BTreeMap::new() };
    let computer = run_program_on_computer(computer, input);
    computer.memory_map.iter().map(|(_, v)| v).sum()
}

fn part_2(input: Vec<Instruction>) -> u64 {
    let computer = Computer { current_mask: "X".to_string(), memory_map: BTreeMap::new() };
    let computer = run_program_on_computer_version_2(computer, input);
    computer.memory_map.iter().map(|(_, v)| v).sum()
}

fn apply_bitmask(value: u64, mask: &str) -> u64 {
    let or_with : String = mask.chars().map(|c| if c == 'X' { '0' } else { c } ).collect();
    let and_with: String = mask.chars().map(|c| if c == 'X' { '1' } else { c } ).collect();
    let or_with = u64::from_str_radix(&or_with, 2).unwrap();
    let and_with = u64::from_str_radix(&and_with, 2).unwrap();
    (value & and_with) | or_with
}

struct Computer {
    current_mask: String,
    memory_map: BTreeMap<usize, u64>,
}

struct MemoryDecoder {
    and_mask: u64,
    or_mask: u64,
    floaters: Vec<usize>,
}

fn mask_to_decoder(mask: &str) -> MemoryDecoder {
    let and_with: String = mask.chars().map(|c| if c == 'X' { '1' } else { '0' } ).collect();
    let and_with = !u64::from_str_radix(&and_with, 2).unwrap();
    let or_with = mask.chars().map(|c| if c == 'X' { '0' } else { c } ).collect::<String>();
    let or_with = u64::from_str_radix(&or_with, 2).unwrap();
    let floaters = mask.chars().rev().enumerate().filter(|(i, c)| *c == 'X').map((|(i, _)| i)).collect();
    MemoryDecoder {
        and_mask: and_with,
        or_mask: or_with,
        floaters
    }
}


// 0bx0x001
// -> 0b000001
// -> 0b001001
// -> 0b100001
// -> 0b101001
// => stretching 00 -> 11 by an idx in a list pos.
fn masks_from_stretched_bits(mask: u64, idxs: Vec<usize>) -> Vec<u64> {
    let mut result = vec![];
    for sub_mask_value in 0..(2usize.pow(idxs.len() as u32)) {
        let mut stretched_sub_mask: u64 = 0;
        let sub_mask_value = sub_mask_value as u64;
        for (i, idx) in idxs.iter().enumerate() {
            stretched_sub_mask |= ((sub_mask_value & (1 << i)) >> i) << idx;
        }
        result.push(mask | stretched_sub_mask);
    }
    result
}

fn addresses_decoded(address: u64, decoder: MemoryDecoder) -> Vec<u64> {
    let base = (address | decoder.or_mask) & decoder.and_mask;
    masks_from_stretched_bits(base, decoder.floaters)
}


fn run_program_on_computer(mut computer: Computer, program: Vec<Instruction>) -> Computer {
    for instruction in program.iter() {
        match instruction {
            Instruction::Mask(mask) => computer.current_mask = mask.clone(),
            Instruction::MemorySet(location, value) => {
                computer.memory_map.insert(*location, apply_bitmask(*value, &computer.current_mask));
            }
        }
    }
    computer
}

fn run_program_on_computer_version_2(mut computer: Computer, program: Vec<Instruction>) -> Computer {
    for instruction in program.iter() {
        match instruction {
            Instruction::Mask(mask) => computer.current_mask = mask.clone(),
            Instruction::MemorySet(location, value) => {
                let decoder = mask_to_decoder(&computer.current_mask);
                let memory_locations = addresses_decoded(*location as u64, decoder);
                for x in memory_locations {
                    computer.memory_map.insert(x as usize, *value);
                }
            }
        }
    }
    computer
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day14").unwrap());
        assert_eq!(part_1(input), 165);
    }

    #[test]
    fn test_part_2() {
        let input = parse_input(&fs::read_to_string("./test_inputs/day14_2.txt").unwrap());
        assert_eq!(part_2(input), 208);
    }

    #[test]
    fn test_masking() {
        assert_eq!(apply_bitmask(11, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"), 73);
        assert_eq!(apply_bitmask(101, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"), 101);
        assert_eq!(apply_bitmask(0, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"), 64)
    }
    #[test]
    fn test_parsing() {
        let input = "mask = 0010X01001X010000110100000X000010X11\n\
                            mem[41717] = 288\n\
                            mem[54146] = 1656\n\
                            mem[30135] = 4799584";
        let correct = vec![
            Instruction::Mask("0010X01001X010000110100000X000010X11".to_string()),
            Instruction::MemorySet(41717, 288),
            Instruction::MemorySet(54146, 1656),
            Instruction::MemorySet(30135, 4799584),
        ];

        assert_eq!(parse_input(input), correct);
    }

    #[test]
    fn test_address_decoded() {
        let decoder = mask_to_decoder("000000000000000000000000000000X1001X");
        assert_eq!(addresses_decoded(42, decoder), vec![26, 27, 58, 59]);
    }

    #[test]
    fn test_masks_from_stretched_bits() {
        let correct = vec![
            0b0000,
            0b0001,
            0b0010,
            0b0011,
            0b1000,
            0b1001,
            0b1010,
            0b1011,
        ];
        let result = masks_from_stretched_bits(0x0, vec![0, 1, 3]);
        assert_eq!(result, correct);

        let correct = vec![
            0b0100,
            0b0101,
            0b0110,
            0b0111,
            0b1100,
            0b1101,
            0b1110,
            0b1111,
        ];
        let result = masks_from_stretched_bits(0b0100, vec![0, 1, 3]);
        assert_eq!(result, correct);
    }
}