use std::fs;

fn main() {
    let input = fs::read_to_string("day1_input.txt").unwrap();
    let vals: Vec<i32> = input.split_whitespace().filter_map(|s| s.parse().ok()).collect();
    println!("Thing is {}", part_one(&vals).unwrap());
}



fn part_one(vals: &[i32]) -> Result<i32, ()> {
    let mut a = vals[0];
    let mut offset = 1;
    while offset < vals.len() {
        for b in vals[offset..vals.len()].iter() {
            if (a + b) == 2020 {
                return Ok(a * b) ;
            }
        }
        a = vals[offset];
        offset += 1;
    }
    Err(())
}

fn part_two(vals: &mut[Option<i32>]) -> Result<i32, ()> {
    let mut a = vals[0];
    let mut offset = 1;
    while offset < vals.len() {
        for b in vals[offset..vals.len()].iter() {
            if (a + b) == 2020 {
                return Ok(a * b) ;
            }
        }
        a = vals[offset];
        offset += 1;
    }
    Err(())
}

#[test]
fn test_part_one() {
    let vals = [
        1721,
        979,
        366,
        299,
        675,
        1456,
    ];

    assert_eq!(part_one(&vals).unwrap(), 514579);
}

#[test]
fn test_part_two() {
    let vals = [
        1721,
        979,
        366,
        299,
        675,
        1456,
    ];

    assert_eq!(part_two(&vals).unwrap(), 241861950)
}