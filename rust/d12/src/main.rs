use std::io::{self, Read};
use regex::Regex;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let input = get_input();
    let instructions = parse_instructions(input);

    let mut ship = Ship {
        x: 0,
        y: 0,
        direction: 0
    };

    for instruction in instructions.iter() {
        ship = execute_instruction(ship, *instruction);
    }

    println!("[Part One] Solution: {}", ship.x.abs() + ship.y.abs());

    let mut ship = Ship {
        x: 0,
        y: 0,
        direction: 0
    };

    let mut waypoint = Waypoint {
        x: 10,
        y: 1
    };

    for instruction in instructions.iter() {
        let executed = execute_instruction_2(ship, waypoint, *instruction);

        ship = executed.0;
        waypoint = executed.1;
    }

    println!("[Part Two] Solution: {}",  ship.x.abs() + ship.y.abs());

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
    let re = Regex::new(r"^(\w)(\d+)$").unwrap();

    return input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();

            let command = match &caps[1] {
                "N" => Command::North,
                "S" => Command::South,
                "E" => Command::East,
                "W" => Command::West,
                "L" => Command::Left,
                "R" => Command::Right,
                "F" => Command::Forward,
                _ => panic!("Unexpected command"),
            };

            let value: i32 = caps[2].parse::<i32>().unwrap();

            return Instruction { command, value };
        })
        .collect();
}

fn execute_instruction(ship: Ship, instruction: Instruction) -> Ship {
    return match instruction.command {
        Command::North => Ship {
            x: ship.x,
            y: ship.y + instruction.value,
            direction: ship.direction
        },
        Command::South => Ship {
            x: ship.x,
            y: ship.y - instruction.value,
            direction: ship.direction
        },
        Command::East => Ship {
            x: ship.x + instruction.value,
            y: ship.y,
            direction: ship.direction
        },
        Command::West => Ship {
            x: ship.x - instruction.value,
            y: ship.y,
            direction: ship.direction
        },
        Command::Left => Ship {
            x: ship.x,
            y: ship.y,
            direction: (ship.direction - instruction.value + 360) % 360
        },
        Command::Right => Ship {
            x: ship.x,
            y: ship.y,
            direction: (ship.direction + instruction.value) % 360
        },
        Command::Forward => Ship {
            x: match ship.direction % 360 {
                0 => ship.x + instruction.value,
                180 => ship.x - instruction.value,
                _ => ship.x
            },
            y: match ship.direction % 360 {
                90 => ship.y - instruction.value,
                270 => ship.y + instruction.value,
                _ => ship.y
            },
            direction: ship.direction
        },
    };
}

fn execute_instruction_2(ship: Ship, waypoint: Waypoint, instruction: Instruction) -> (Ship, Waypoint) {
    return match instruction.command {
        Command::North => (ship, Waypoint {
            x: waypoint.x,
            y: waypoint.y + instruction.value
        }),
        Command::South => (ship, Waypoint {
            x: waypoint.x,
            y: waypoint.y - instruction.value
        }),
        Command::East => (ship, Waypoint {
            x: waypoint.x + instruction.value,
            y: waypoint.y
        }),
        Command::West => (ship, Waypoint {
            x: waypoint.x - instruction.value,
            y: waypoint.y
        }),
        Command::Left => (ship, 
            match instruction.value {
                90 => Waypoint {
                    x: -waypoint.y,
                    y: waypoint.x
                },
                180 => Waypoint {
                    x: -waypoint.x,
                    y: -waypoint.y
                },
                270 => Waypoint {
                    x: waypoint.y,
                    y: -waypoint.x
                },
                _ => panic!("Unexpected parameter")
            }
        ),
        Command::Right => (ship, 
            match instruction.value {
            90 => Waypoint {
                x: waypoint.y,
                y: -waypoint.x
            },
            180 => Waypoint {
                x: -waypoint.x,
                y: -waypoint.y
            },
            270 => Waypoint {
                x: -waypoint.y,
                y: waypoint.x
            },
            _ => panic!("Unexpected parameter")
        }),
        Command::Forward => (Ship {
            x: ship.x + (waypoint.x * instruction.value),
            y: ship.y + (waypoint.y * instruction.value),
            direction: ship.direction
        }, waypoint)
    };
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Command {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Instruction {
    command: Command,
    value: i32
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Ship {
    x: i32,
    y: i32,
    direction: i32
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Waypoint {
    x: i32,
    y: i32
}