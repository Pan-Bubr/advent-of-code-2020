use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Read};
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let input = get_input();

    let bags = parse_details(input);

    // Part 1 - How many bags can hold a shiny gold bag

    let mut bags_able_to_carry_shiny_gold: HashSet<String> = HashSet::new();
    let mut suspicious_bags: Vec<String> = vec!["shiny gold".to_string()];

    while suspicious_bags.len() > 0 {
        let suspicious_bag = suspicious_bags.pop().unwrap();

        for bag in bags.values() {
            if bag.contains.iter().any(|(_, name)| *name == suspicious_bag) {
                bags_able_to_carry_shiny_gold.insert(bag.name.clone());
                suspicious_bags.push(bag.name.clone());
            };
        }
    }

    println!(
        "[Part One] Solution: {}",
        bags_able_to_carry_shiny_gold.len()
    );

    // Part 2 - How many bags are inside shiny gold bag

    let bag_count = calculate_bags(&bags, "shiny gold".to_string());

    println!("[Part Two] Solution: {}", bag_count - 1);

    let duration = start.elapsed();
    println!("\nCalculated in: {:?}", duration);
}

fn get_input() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();
    return input;
}

fn parse_details(input: String) -> HashMap<String, Bag> {
    let bag_regex = Regex::new(r"^(\w+ \w+) bags contain").unwrap();
    let contain_regex = Regex::new(r"(\d+) (\w+ \w+) bag").unwrap();

    let mut bag_map: HashMap<String, Bag> = HashMap::new();
    input.lines().for_each(|line| {
        let bag_name: String = bag_regex.captures(line).unwrap()[1].to_string();

        let bag_contains: Vec<(u32, String)> = contain_regex
            .captures_iter(line)
            .map(|cap| (cap[1].parse::<u32>().unwrap(), cap[2].to_string()))
            .collect();

        bag_map.insert(
            bag_name.clone(),
            Bag {
                name: bag_name.clone(),
                contains: bag_contains,
            },
        );
    });
    return bag_map;
}

fn calculate_bags(bag_list: &HashMap<String, Bag>, bag_name: String) -> u32 {
    let bag = bag_list.get(&bag_name).unwrap();

    return bag
        .contains
        .iter()
        .map(|(amount, contained_bag)| {
            return amount * calculate_bags(&bag_list, contained_bag.clone());
        })
        .sum::<u32>()
        + 1;
}

struct Bag {
    name: String,
    contains: Vec<(u32, String)>,
}
