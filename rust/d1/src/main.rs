use std::io::{self, Read};

fn main() {
    let input = get_input();
    let expenses: Vec<i32> = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

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

    // Part 2

    let mut part2_pairs: Vec<(i32, i32)> = Vec::new();
    let mut solution2: i32 = 0;

    for a in &expenses {
        for b in &expenses {
            if a != b && a < b {
                part2_pairs.push((*a, *b));
            }
        }
    }

    'part2: for c in &expenses {
        for (a, b) in &part2_pairs {
            if *c != *a && *c != *b && *c + *a + *b == 2020 {
                solution2 = a * b * c;
                break 'part2;
            }
        }
    }

    println!("[Part Two] Solution: {}", solution2);
}

fn get_input() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();
    return input;
}
