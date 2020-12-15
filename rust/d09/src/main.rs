use std::io::{self, Read};
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let input = get_input();
    let numbers: Vec<i64> = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    let invalid_number = invalid_sum(numbers.clone(), 25);

    println!("[Part One] Solution: {:?}", invalid_number);

    let range_sum = find_sum(numbers.clone(), invalid_number);
    println!(
        "[Part Two] Solution: {:?}",
        range_sum.iter().min().unwrap() + range_sum.iter().max().unwrap()
    );

    let duration = start.elapsed();
    println!("\nCalculated in: {:?}", duration);
}

fn get_input() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();
    return input;
}

fn invalid_sum(input: Vec<i64>, preamble_length: usize) -> i64 {
    let mut preamble = input[0..preamble_length].to_vec();

    for tested_number in input[preamble_length..].iter() {
        let sum_completions: Vec<i64> =
            preamble.clone().iter().map(|n| tested_number - n).collect();

        let valid = sum_completions.iter().any(|n| preamble.contains(n));

        if !valid {
            return *tested_number;
        }
        preamble.rotate_left(1);
        preamble.pop();
        preamble.push(*tested_number);
    }

    panic!("Invalid input");
}

fn find_sum(input: Vec<i64>, sums_to: i64) -> Vec<i64> {
    for i in 2..input.len() {
        for num in 1..input.len() - i {
            let tested_range = input[num..num + i].to_vec();
            let range_sum: i64 = tested_range.iter().sum();

            if range_sum == sums_to {
                return tested_range;
            }
        }
    }

    return Vec::new();
}
