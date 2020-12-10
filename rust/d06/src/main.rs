use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let input = get_input();

    let groups: Vec<(usize, HashMap<char, u32>)> = parse_declatarions(input);

    let items_mentioned = groups.iter().fold(0, |i, (_, group)| {
        return i + group.len();
    });

    println!("[Part One] Solution: {}", items_mentioned);

    let items_agreed = groups.iter().fold(0, |i, (group_size, group)| {
        return group.values().fold(i, |i, choice_count| {
            if *choice_count as usize == *group_size {
                i + 1
            } else {
                i
            }
        });
    });

    println!("[Part Two] Solution: {}", items_agreed);
}

fn get_input() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();
    return input;
}

fn parse_declatarions(input: String) -> Vec<(usize, HashMap<char, u32>)> {
    return input
        .split("\n\n")
        .map(|group| {
            let mut group_items = HashMap::new();

            for person in group.lines() {
                for c in person.chars() {
                    *group_items.entry(c).or_insert(0) += 1;
                }
            }

            return (group.lines().count(), group_items);
        })
        .collect();
}
