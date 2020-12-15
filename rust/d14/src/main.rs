use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let input = get_input();
    let instructions = parse_instructions(input);
    let memory = execute_instructions(instructions);

    let sum = memory.values().sum::<u64>();

    println!("[Part One] Solution: {}", sum);
    println!("[Part Two] Solution: {}", 0);

    let duration = start.elapsed();
    println!("\nCalculated in: {:?}", duration);
}

fn get_input() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();
    return input;
}

fn parse_instructions(input: String) -> Vec<(Mask, Vec<MemoryInput>)> {
    let re_mask = Regex::new(r"^mask = (\w+)$").unwrap();
    let re_mem = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    let mut ret: Vec<(Mask, Vec<MemoryInput>)> = Vec::new();

    let mut current_mask: (Mask, Vec<MemoryInput>) = (
        Mask {
            positive: 0,
            negative: 0,
        },
        Vec::new(),
    );

    for line in input.lines() {
        if re_mask.is_match(line) {
            ret.push(current_mask);

            let mask = &re_mask.captures(line).unwrap()[1];

            let parsed: (String, String) = mask
                .chars()
                .map(|c| match c {
                    '0' => ('0', '0'),
                    '1' => ('1', '1'),
                    'X' => ('0', '1'),
                    _ => panic!("Unexpected"),
                })
                .fold(
                    (String::new(), String::new()),
                    |(mut pos, mut neg), (pos_c, neg_c)| {
                        pos.push(pos_c);
                        neg.push(neg_c);
                        return (pos, neg);
                    },
                );

            current_mask = (
                Mask {
                    positive: u64::from_str_radix(&parsed.0, 2).unwrap(),
                    negative: u64::from_str_radix(&parsed.1, 2).unwrap(),
                },
                Vec::new(),
            )
        } else if re_mem.is_match(line) {
            let mask = &re_mem.captures(line).unwrap();

            current_mask.1.push(MemoryInput {
                address: mask[1].parse::<u64>().unwrap(),
                value: mask[2].parse::<u64>().unwrap(),
            })
        }
    }

    ret.rotate_left(1);
    ret.pop();
    ret.push(current_mask);

    return ret;
}

fn execute_instructions(instructions: Vec<(Mask, Vec<MemoryInput>)>) -> HashMap<u64, u64> {
    let mut mem = HashMap::new();

    for (mask, mem_inputs) in instructions {
        for input in mem_inputs {
            let value = input.value & mask.negative | mask.positive;

            mem.insert(input.address, value);
        }
    }

    return mem;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Mask {
    negative: u64,
    positive: u64,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Mask {
    negative: u64,
    positive: u64,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct MemoryInput {
    address: u64,
    value: u64,
}
