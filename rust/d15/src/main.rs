use std::collections::HashMap;
use std::io::{self, Read};
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let input = get_input()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut numbers: HashMap<usize, usize> = HashMap::new();

    for i in 0..input.len() - 1 {
        numbers.insert(*input.get(i).unwrap(), i);
    }

    let mut last_shout = *input.last().unwrap();

    for i in input.len() - 1..2020 - 1 {
        if i % 300000 == 0 {
            dbg!(30000000 - i);
        }
        let insert;
        let shout;
        match numbers.get(&last_shout) {
            Some(time) => {
                insert = (last_shout, i);
                shout = i - time;
            }
            None => {
                insert = (last_shout, i);
                shout = 0;
            }
        }

        numbers.insert(insert.0, insert.1);
        last_shout = shout;
    }

    println!("[Part One] Solution: {}", last_shout);

    let mut numbers: HashMap<usize, usize> = HashMap::new();

    for i in 0..input.len() - 1 {
        numbers.insert(*input.get(i).unwrap(), i);
    }

    let mut last_shout = *input.last().unwrap();

    for i in input.len() - 1..30000000 - 1 {
        // if i % 1000000 == 0 {dbg!(30000000 - i); }
        let insert;
        let shout;
        match numbers.get(&last_shout) {
            Some(time) => {
                insert = (last_shout, i);
                shout = i - time;
            }
            None => {
                insert = (last_shout, i);
                shout = 0;
            }
        }

        numbers.insert(insert.0, insert.1);
        last_shout = shout;
    }
    println!("[Part Two] Solution: {}", last_shout);

    let duration = start.elapsed();
    println!("\nCalculated in: {:?}", duration);
}

fn get_input() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();
    return input;
}
