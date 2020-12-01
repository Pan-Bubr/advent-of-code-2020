use std::io::{self, Read};

fn main() {
    let input = get_input();
    let expenses: Vec<i32> = input.lines().map(|line| line.parse::<i32>().unwrap()).collect();

    // Part 1
    let mut solution1 = 0;

    'part1: for a in &expenses {
        for b in &expenses {
            if *b == 2020 - *a {
                solution1 = a * b;
                break 'part1;
            }
        }
    }

    println!("[Part One] Solution: {}", solution1);
}

fn get_input() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();
    return input;
}
