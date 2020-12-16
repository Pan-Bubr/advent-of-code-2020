use std::io::{self, Read};
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let input = get_input();

    let mut lines = input.lines();
    let timestamp = lines.next().unwrap().parse::<i64>().unwrap();
    let buses: Vec<&str> = lines.next().unwrap().split(',').collect();

    let bus: (i64, i64) = buses
        .iter()
        .filter_map(|c| c.parse::<i64>().ok())
        .map(|n| (n, timestamp + (n - timestamp % n)))
        .min_by_key(|bus| bus.1)
        .unwrap();

    println!("[Part One] Solution: {}", bus.0 * (bus.1 - timestamp));

    let offset_buses: Vec<(i64, i64)> = buses
        .iter()
        .enumerate()
        .filter(|(_, b)| b.parse::<i64>().is_ok())
        .map(|(i, b)| (b.parse::<i64>().unwrap(), i as i64))
        .collect();

    let prod: i64 = offset_buses.iter().map(|(b, _)| b).product();
    let sum = offset_buses
        .iter()
        .map(|(b, a)| -a * (prod / b) * mod_inv(prod / b, *b).unwrap())
        .sum::<i64>();
    println!("[Part Two] Solution: {}", sum.rem_euclid(prod));

    let duration = start.elapsed();
    println!("\nCalculated in: {:?}", duration);
}

fn get_input() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();
    return input;
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
