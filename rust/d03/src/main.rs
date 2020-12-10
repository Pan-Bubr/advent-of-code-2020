use std::io::{self, Read};

fn main() {
    let input = get_input();
    let area: Vec<Vec<Field>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Field::Dirt,
                    '#' => Field::Tree,
                    _ => panic!("Error"),
                })
                .collect()
        })
        .collect();

    let trees_on_first_path = tree_count_on_path(&area, 1, 3);

    println!("[Part One] Solution: {}", trees_on_first_path);

    let trees_on_all_paths = vec![
        tree_count_on_path(&area, 1, 1),
        tree_count_on_path(&area, 1, 3),
        tree_count_on_path(&area, 1, 5),
        tree_count_on_path(&area, 1, 7),
        tree_count_on_path(&area, 2, 1),
    ]
    .into_iter()
    .fold(1, |a, b| a * b);

    println!("[Part Two] Solution: {}", trees_on_all_paths);
}

fn get_input() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();
    return input;
}

fn tree_count_on_path(area: &Vec<Vec<Field>>, step_down: usize, step_right: usize) -> usize {
    let mut trees_encountered = 0;

    for (i, line) in area.iter().step_by(step_down).enumerate() {
        if line[i * step_right % line.len()] == Field::Tree {
            trees_encountered += 1;
        }
    }

    return trees_encountered;
}

#[derive(PartialEq)]
enum Field {
    Tree,
    Dirt,
}
