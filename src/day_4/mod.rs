use anyhow::Context;
use itertools::Itertools;
use std::convert::TryFrom;
use std::error::Error;
use std::i32;
use std::{fmt, fs};

pub fn solve_part_1() -> Result<(), ()> {
    println!(
        "{}",
        parse_input(&fs::read_to_string("inputs/day4.txt").unwrap())
            .iter()
            .count()
    );
    Ok(())
}

fn parse_input(input: &str) -> Vec<Entry> {
    input
        .split("\n\n")
        .filter_map(|item| Entry::try_from(item).ok())
        .collect()
}

#[derive(Debug)]
struct Entry {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

#[derive(Debug)]
struct GenericError {}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "I'm a generic Error!")
    }
}
impl Error for GenericError {}

impl Entry {
    fn new() -> Entry {
        Entry {
            byr: String::new(),
            iyr: String::new(),
            eyr: String::new(),
            hgt: String::new(),
            hcl: String::new(),
            ecl: String::new(),
            pid: String::new(),
            cid: None,
        }
    }

    //byr (Birth Year) - four digits; at least 1920 and at most 2002.
    fn parse_byr(byr: &str) -> anyhow::Result<()> {
        let val = byr.parse::<i32>()?;
        if (1920..=2002).contains(&val) {
            Ok(())
        } else {
            Err(anyhow::Error::new(GenericError {}))
        }
    }

    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    fn parse_iyr(iyr: &str) -> anyhow::Result<()> {
        let val = iyr.parse::<i32>()?;
        if (2010..=2020).contains(&val) {
            Ok(())
        } else {
            Err(anyhow::Error::new(GenericError {}))
        }
    }

    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    fn parse_eyr(eyr: &str) -> anyhow::Result<()> {
        let val = eyr.parse::<i32>()?;
        if (2020..=2030).contains(&val) {
            Ok(())
        } else {
            Err(anyhow::Error::new(GenericError {}))
        }
    }

    // hgt (Height) - a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.
    fn parse_hgt(hgt: &str) -> anyhow::Result<()> {
        let val = hgt[0..(hgt.len() - 2)].parse::<i32>()?;
        match &hgt[(hgt.len() - 2)..(hgt.len())] {
            "cm" => {
                if (150..=193).contains(&val) {
                    Ok(())
                } else {
                    Err(anyhow::Error::new(GenericError {}))
                }
            }
            "in" => {
                if (59..=76).contains(&val) {
                    Ok(())
                } else {
                    Err(anyhow::Error::new(GenericError {}))
                }
            }
            _ => Err(anyhow::Error::new(GenericError {})),
        }
    }

    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    fn parse_hcl(hcl: &str) -> anyhow::Result<()> {
        if hcl.chars().next().unwrap_or(' ') != '#' {
            return Err(anyhow::Error::new(GenericError {}));
        }
        let val = i32::from_str_radix(&hcl[1..(hcl.len())], 16)?;
        if val < 0xffffff {
            Ok(())
        } else {
            Err(anyhow::Error::new(GenericError {}))
        }
    }

    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    fn parse_ecl(ecl: &str) -> anyhow::Result<()> {
        match ecl {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Ok(()),
            _ => Err(anyhow::Error::new(GenericError {})),
        }
    }

    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    fn parse_pid(pid: &str) -> anyhow::Result<()> {
        let val = pid.parse::<u64>()?;
        if pid.len() != 9 {
            return Err(anyhow::Error::new(GenericError {}));
        }
        if val < 999_999_999 {
            Ok(())
        } else {
            Err(anyhow::Error::new(GenericError {}))
        }
    }
}

impl TryFrom<&str> for Entry {
    type Error = anyhow::Error;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut found = vec![false; 7];
        let mut entry = Entry::new();
        for item in input.split_whitespace() {
            let (key, value) = item
                .split(':')
                .next_tuple()
                .expect("Found an entry without two parts separated by a colon");
            match key {
                "byr" => {
                    found[0] = true;
                    entry.byr = value.to_string();
                    if let Err(_) = Entry::parse_byr(&entry.byr) {
                        return Err(anyhow::Error::new(GenericError {}));
                    }
                }
                "iyr" => {
                    found[1] = true;
                    entry.iyr = value.to_string();
                    if let Err(_) = Entry::parse_iyr(&entry.iyr) {
                        return Err(anyhow::Error::new(GenericError {}));
                    }
                }
                "eyr" => {
                    found[2] = true;
                    entry.eyr = value.to_string();
                    if let Err(_) = Entry::parse_eyr(&entry.eyr) {
                        return Err(anyhow::Error::new(GenericError {}));
                    }
                }
                "hgt" => {
                    found[3] = true;
                    entry.hgt = value.to_string();
                    if let Err(_) = Entry::parse_hgt(&entry.hgt) {
                        return Err(anyhow::Error::new(GenericError {}));
                    }
                }
                "hcl" => {
                    found[4] = true;
                    entry.hcl = value.to_string();
                    if let Err(_) = Entry::parse_hcl(&entry.hcl) {
                        return Err(anyhow::Error::new(GenericError {}));
                    }
                }
                "ecl" => {
                    found[5] = true;
                    entry.ecl = value.to_string();
                    if let Err(_) = Entry::parse_ecl(&entry.ecl) {
                        return Err(anyhow::Error::new(GenericError {}));
                    }
                }
                "pid" => {
                    found[6] = true;
                    entry.pid = value.to_string();
                    if let Err(_) = Entry::parse_pid(&entry.pid) {
                        return Err(anyhow::Error::new(GenericError {}));
                    }
                }
                "cid" => {
                    entry.cid = Some(value.to_string());
                }
                _ => panic!("Unknown key found"),
            }
        }

        if found.iter().all(|v| *v) {
            Ok(entry)
        } else {
            Err(anyhow::Error::new(GenericError {})).with_context(|| "Entry did not parse")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_test_input(file: &str) -> Vec<Entry> {
        parse_input(&fs::read_to_string(file).unwrap())
    }

    #[test]
    fn test_part_1() {
        assert_eq!(read_test_input("test_inputs/day4").iter().count(), 2);
    }

    #[test]
    fn check_valid_passports() {
        assert_eq!(
            read_test_input("test_inputs/passports/valid_passports")
                .iter()
                .count(),
            4
        );
    }

    #[test]
    fn check_invalid_passports() {
        assert_eq!(
            read_test_input("test_inputs/passports/invalid_passports")
                .iter()
                .count(),
            0
        );
    }

    #[test]
    fn test_example_validity() {
        assert!(Entry::parse_byr("2002").is_ok());
        assert!(Entry::parse_byr("2003").is_err());
        // hgt valid:   60in
        // hgt valid:   190cm
        // hgt invalid: 190in
        // hgt invalid: 190

        assert!(Entry::parse_hgt("60in").is_ok());
        assert!(Entry::parse_hgt("190cm").is_ok());
        assert!(Entry::parse_hgt("190in").is_err());
        assert!(Entry::parse_hgt("190").is_err());

        // hcl valid:   #123abc
        // hcl invalid: #123abz
        // hcl invalid: 123abc
        assert!(Entry::parse_hcl("#123abc").is_ok());
        assert!(Entry::parse_hcl("#123abz").is_err());
        assert!(Entry::parse_hcl("123abc").is_err());

        // ecl valid:   brn
        // ecl invalid: wat
        assert!(Entry::parse_ecl("brn").is_ok());
        assert!(Entry::parse_ecl("wat").is_err());

        // pid valid:   000000001
        // pid invalid: 0123456789
        assert!(Entry::parse_pid("000000001").is_ok());
        assert!(Entry::parse_pid("0123456789").is_err());
    }
}
