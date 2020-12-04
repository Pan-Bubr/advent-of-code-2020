use regex::Regex;
use regex::RegexSet;
use std::io::{self, Read};

fn main() {
    let input = get_input();

    let passports: Vec<String> = parse_records(input);

    let initial_checks = RegexSet::new(&[
        r"byr:(#?\w+)",
        r"iyr:(#?\w+)",
        r"eyr:(#?\w+)",
        r"hgt:(#?\w+)",
        r"hcl:(#?\w+)",
        r"ecl:(#?\w+)",
        r"pid:(#?\w+)",
    ])
    .unwrap();

    let checked_passports: Vec<String> = passports
        .into_iter()
        .filter(|passport| {
            let matches = initial_checks.matches(passport).into_iter().count();

            return matches == 7;
        })
        .collect();

    println!("[Part One] Solution: {}", checked_passports.len());

    let re_birth_year = Regex::new(r"byr:(\d{4})").unwrap();
    let re_issue_year = Regex::new(r"iyr:(\d{4})").unwrap();
    let re_expiration_year = Regex::new(r"eyr:(\d{4})").unwrap();
    let re_height = Regex::new(r"hgt:(\w+)").unwrap();
    let re_hair_color = Regex::new(r"hcl:(#[0-9a-f]{6})").unwrap();
    let re_eye_color = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
    let re_passport_id = Regex::new(r"pid:(\d+)").unwrap();

    let re_cm = Regex::new(r"(\d+)cm").unwrap();
    let re_in = Regex::new(r"(\d+)in").unwrap();

    let validated_passports: Vec<String> = checked_passports
        .into_iter()
        .filter(|passport| {
            // Birth Year in 1920-2020
            let birth_year = re_birth_year.captures(passport).unwrap()[1]
                .parse::<u32>()
                .unwrap();
            if !(1920..=2002).contains(&birth_year) {
                return false;
            }

            // Issue Year in 2010-2020
            let issue_year = re_issue_year.captures(passport).unwrap()[1]
                .parse::<u32>()
                .unwrap();
            if !(2010..=2020).contains(&issue_year) {
                return false;
            }

            // Expiration Year in 2020-2030
            let expiration_year = re_expiration_year.captures(passport).unwrap()[1]
                .parse::<u32>()
                .unwrap();
            if !(2020..=2030).contains(&expiration_year) {
                return false;
            }

            // Height 150-193 cm or 59-76 inches
            let height = re_height.captures(passport).unwrap()[1]
                .parse::<String>()
                .unwrap();
            if re_cm.is_match(&height) {
                let height_in_cm = re_cm.captures(&height).unwrap()[1].parse::<u32>().unwrap();
                if !(150..=193).contains(&height_in_cm) {
                    return false;
                }
            } else if re_in.is_match(&height) {
                let height_in_in = re_in.captures(&height).unwrap()[1].parse::<u32>().unwrap();
                if !(59..=76).contains(&height_in_in) {
                    return false;
                }
            } else {
                return false;
            }

            // Hair Color in hex
            if !re_hair_color.is_match(&passport) {
                return false;
            }

            // Eye Color from enum
            if !re_eye_color.is_match(&passport) {
                return false;
            }

            // Passport Id 9 digits
            if !re_passport_id.is_match(&passport) {
                return false;
            } else {
                let passport_id = re_passport_id.captures(&passport).unwrap()[1]
                    .parse::<String>()
                    .unwrap();
                if passport_id.len() != 9 {
                    return false;
                }
            }

            return true;
        })
        .collect();

    println!("[Part Two] Solution: {}", validated_passports.len());
}

fn get_input() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();
    return input;
}

fn parse_records(input: String) -> Vec<String> {
    return input
        .split("\n\n")
        .map(|record| {
            record
                .lines()
                .map(|line| String::from(line))
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect();
}
