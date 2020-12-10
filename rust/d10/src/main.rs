use std::collections::HashMap;
use std::io::{self, Read};
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let input = get_input();
    let mut unsorted_numbers: Vec<u64> = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();
    unsorted_numbers.sort();
    let numbers = unsorted_numbers.clone();

    let (by_1, by_3) = calculate_jolt_differences(&numbers);
    println!("[Part One] Solution: {:?}", by_1 * by_3);

    let arrangements = count_arrangements_backwards(numbers.clone());

    println!("[Part Two] Solution: {:?}", arrangements);

    let duration = start.elapsed();
    println!("\nCalculated in: {:?}", duration);
}

fn get_input() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();
    return input;
}

fn calculate_jolt_differences(numbers: &Vec<u64>) -> (u64, u64) {
    let mut done = false;
    let mut jolt_differences: (u64, u64) = (0, 1);

    let mut current_joltage = 0;

    while !done {
        if numbers.contains(&(current_joltage + 1)) {
            jolt_differences.0 += 1;
            current_joltage += 1;
        } else if numbers.contains(&(current_joltage + 3)) {
            jolt_differences.1 += 1;
            current_joltage += 3;
        } else {
            done = true;
        }
    }

    return jolt_differences;
}

// First Attempt - Infinite time
// fn count_arrangements(numbers: Vec<u64>, current_joltage: u64) -> u64 {
//     let mut arrangements_found = 0;
//     let numbers = numbers.clone();

//     if numbers.len() == 0 {
//         return 1;
//     }

//     if numbers.contains(&(current_joltage + 1)) {
//         let id = numbers
//             .iter()
//             .position(|&r| r == current_joltage + 1)
//             .unwrap();

//         let shorter_numbers = numbers[id + 1..].to_vec();
//         arrangements_found += count_arrangements(shorter_numbers, current_joltage + 1);
//     }

//     if numbers.contains(&(current_joltage + 2)) {
//         let id = numbers
//             .iter()
//             .position(|&r| r == current_joltage + 2)
//             .unwrap();
//         let shorter_numbers = numbers[id + 1..].to_vec();
//         arrangements_found += count_arrangements(shorter_numbers, current_joltage + 2);
//     }

//     if numbers.contains(&(current_joltage + 3)) {
//         let id = numbers
//             .iter()
//             .position(|&r| r == current_joltage + 3)
//             .unwrap();
//         let shorter_numbers = numbers[id + 1..].to_vec();
//         arrangements_found += count_arrangements(shorter_numbers, current_joltage + 3);
//     }

//     return arrangements_found;
// }

// Second Attemp - Reverse method
fn count_arrangements_backwards(numbers: Vec<u64>) -> u64 {
    let mut numbers = numbers.clone();
    numbers.reverse();
    numbers.push(0);

    let mut arrangements: HashMap<u64, u64> = HashMap::new();

    arrangements.insert(numbers[0], 1);

    for n in numbers[1..].iter() {
        for i in 1..=3 {
            match arrangements.get(&(n + i)).copied() {
                Some(value) => match arrangements.get(&n).copied() {
                    Some(new_value) => {
                        arrangements.insert(*n, value + new_value);
                    }
                    None => {
                        arrangements.insert(*n, value);
                    }
                },
                None => {}
            };
        }
    }

    return *arrangements.get(&0).unwrap();
}
