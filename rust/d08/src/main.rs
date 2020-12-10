use regex::Regex;
use std::collections::HashSet;
use std::io::{self, Read};
use std::time::Instant;

fn main() {
    let input = get_input();
    let instructions = parse_instructions(input);

    let start = Instant::now();
    let (value_after_break, _, _) = execute_instruction_list(instructions.clone());

    println!("[Part One] Solution: {}", value_after_break);

    let mut starting_switch_nop_jmp = 0;
    let final_accumulator;

    loop {
        let mut switch_nop_jmp = starting_switch_nop_jmp;
        let mut skipped = false;
        let instructions_with_nop_jmp_switch: Vec<Instruction> = instructions
            .clone()
            .iter()
            .map(|instruction| {
                if !skipped {
                    if switch_nop_jmp != 0 {
                        switch_nop_jmp -= 1;
                    } else {
                        return match instruction.command {
                            Command::JMP => {
                                skipped = true;
                                Instruction {
                                    command: Command::NOP,
                                    value: instruction.value,
                                }
                            }
                            Command::NOP => {
                                skipped = true;
                                Instruction {
                                    command: Command::JMP,
                                    value: instruction.value,
                                }
                            }
                            _ => *instruction,
                        };
                    }
                }

                return *instruction;
            })
            .collect();

        let (accumulator, _, success) = execute_instruction_list(instructions_with_nop_jmp_switch);

        if success {
            final_accumulator = accumulator;
            break;
        }

        starting_switch_nop_jmp += 1;
    }

    println!("[Part Two] Solution: {}", final_accumulator);

    let duration = start.elapsed();
    println!("\nCalculated in: {:?}", duration);
}

fn get_input() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();
    return input;
}

fn parse_instructions(input: String) -> Vec<Instruction> {
    let re = Regex::new(r"^(\w\w\w) (\+|\-)(\d+)$").unwrap();

    return input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();

            let command = match &caps[1] {
                "nop" => Command::NOP,
                "acc" => Command::ACC,
                "jmp" => Command::JMP,
                _ => panic!("Unexpected command"),
            };

            let value: i32 = match &caps[2] {
                "+" => caps[3].parse::<i32>().unwrap(),
                "-" => -caps[3].parse::<i32>().unwrap(),
                _ => panic!("Unexpected sign"),
            };

            return Instruction { command, value };
        })
        .collect();
}

fn execute_instruction_list(instructions: Vec<Instruction>) -> (i32, usize, bool) {
    let mut pointer_history: HashSet<usize> = HashSet::new();
    let mut repeat = false;

    let mut accumulator: (i32, usize) = (0, 0);

    while !repeat {
        pointer_history.insert(accumulator.1);

        accumulator = execute_instruction(accumulator, instructions[accumulator.1]);

        if pointer_history.contains(&accumulator.1) {
            repeat = true;
        }

        if accumulator.1 >= instructions.len() as usize {
            return (accumulator.0, accumulator.1, true);
        }
    }

    return (accumulator.0, accumulator.1, false);
}

fn execute_instruction(
    (accumulator, pointer): (i32, usize),
    instruction: Instruction,
) -> (i32, usize) {
    match instruction.command {
        Command::ACC => (accumulator + instruction.value, pointer + 1),
        Command::NOP => (accumulator, pointer + 1),
        Command::JMP => (accumulator, (pointer as i32 + instruction.value) as usize),
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Command {
    NOP,
    ACC,
    JMP,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Instruction {
    command: Command,
    value: i32,
}
