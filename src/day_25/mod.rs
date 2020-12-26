pub fn solve_part_1() -> Result<(), ()> {
    let door_key = 8987316;
    let (_, card_loop) = part_1((8987316, 14681524));
    println!("{}", transform(door_key, card_loop));
    Ok(())
}

fn part_1((door_key, card_key): (u64, u64)) -> (u64, u64) {
    (transform_with_target(7, door_key), transform_with_target(7, card_key))
}

pub fn solve_part_2() -> Result<(), ()> {
    println!("Merry Christmas!");
    Ok(())
}

fn transform(subject_number: u64, loop_times: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_times {
        value *= subject_number;
        value %= 20_201_227;
    }
    value
}

fn transform_with_target(subject_number: u64, target: u64) -> u64 {
    let mut value = 1;
    for i in 0.. {
        if i % 1_000_000 == 0 {
            print!("X");
        }
        if value == target {
            return i;
        }
        value *= subject_number;
        value %= 20_201_227;
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform() {
        assert_eq!(transform(7, 8), 5_764_801);
    }

    #[test]
    fn test_determine_loop_times() {
        assert_eq!(transform_with_target(7, 5_764_801), 8);
        assert_eq!(transform_with_target(7, 17_807_724), 11);
    }
}
