use std::io::{self, Read};

fn main() {
    let input = get_input();

    let mut seats: Vec<Seat> = input
        .lines()
        .into_iter()
        .map(|line| calculate_seat(line))
        .collect();

    seats.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());

    let max_seat_id = seats.iter().map(|seat| seat.id).into_iter().max().unwrap();

    println!("[Part One] Solution: {}", max_seat_id);

    let mut expected: u32 = seats[0].id;
    for seat in seats {
        if expected == seat.id {
            expected += 1
        } else {
            break;
        }
    }

    println!("[Part Two] Solution: {}", expected);
}

fn get_input() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();
    return input;
}

fn calculate_seat(line: &str) -> Seat {
    let (row, column) = line.split_at(7);

    let row_bin: String = row
        .chars()
        .into_iter()
        .map(|c| match c {
            'F' => '0',
            'B' => '1',
            _ => panic!(),
        })
        .collect::<String>();

    let column_bin: String = column
        .chars()
        .into_iter()
        .map(|c| match c {
            'L' => '0',
            'R' => '1',
            _ => panic!(),
        })
        .collect::<String>();

    let row_number = u32::from_str_radix(&row_bin, 2).unwrap();
    let column_number = u32::from_str_radix(&column_bin, 2).unwrap();

    return Seat {
        row: row_number,
        column: column_number,
        id: row_number * 8 + column_number,
    };
}

#[derive(Debug, Copy, Clone)]
struct Seat {
    row: u32,
    column: u32,
    id: u32,
}
