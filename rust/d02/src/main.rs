use regex::Regex;
use std::io::{self, Read};

fn main() {
    let input = get_input();
    let password_regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    let passwords: Vec<(usize, usize, char, String)> = input
        .lines()
        .map(|line| {
            let caps = password_regex.captures(line).unwrap();

            (
                caps[1].parse::<usize>().unwrap(),
                caps[2].parse::<usize>().unwrap(),
                caps[3].parse::<char>().unwrap(),
                String::from(&caps[4]),
            )
        })
        .collect();

    // Part 1

    let mut valid_password_count = 0;

    for (min, max, letter, password) in &passwords {
        let re = Regex::new(&letter.to_string()).unwrap();
        let count = re.find_iter(&password).count();

        if count >= *min && count <= *max {
            valid_password_count += 1;
        }
    }

    println!("[Part One] Solution: {}", valid_password_count);

    // Part 2

    let mut valid_password_2_count = 0;

    for (a, b, letter, password) in &passwords {
        let char_a = password.chars().nth(a - 1).unwrap();
        let char_b = password.chars().nth(b - 1).unwrap();

        if (char_a == *letter && char_b != *letter) || (char_a != *letter && char_b == *letter) {
            valid_password_2_count += 1;
        }
    }

    println!("[Part Two] Solution: {}", valid_password_2_count);
}

fn get_input() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();
    return input;
}
