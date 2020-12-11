use std::io::{self, Read};
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let input = get_input();

    let mut layout = parse_layout(input.clone());

    loop {
        if !layout.is_crowded() && !layout.has_free_seats() {
            break;
        }

        if layout.has_free_seats() {
            layout.add_seats();
        } else if layout.is_crowded() {
            layout.remove_seats();
        }
    }

    println!("[Part One] Solution: {}", layout.count_seats(Tile::Taken));

    let mut layout = parse_layout(input.clone());
    loop {
        if !layout.is_crowded_2() && !layout.has_free_seats_2() {
            break;
        }

        if layout.has_free_seats_2() {
            layout.add_seats_2();
        } else if layout.is_crowded_2() {
            layout.remove_seats_2();
        }
    }

    // layout.print();

    println!("[Part Two] Solution: {}", layout.count_seats(Tile::Taken));

    let duration = start.elapsed();
    println!("\nCalculated in: {:?}", duration);
}

fn get_input() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();
    return input;
}

fn parse_layout(input: String) -> Layout {
    let rows: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Floor,
                    '#' => Tile::Taken,
                    'L' => Tile::Empty,
                    _ => panic!("Error"),
                })
                .collect()
        })
        .collect();

    return Layout {
        height: rows.len() as i32,
        width: rows[0].len() as i32,
        rows,
    };
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Taken,
    Floor,
}

struct Layout {
    rows: Vec<Vec<Tile>>,
    width: i32,
    height: i32,
}

impl Layout {
    fn get_tile(&self, x: i32, y: i32) -> Option<&Tile> {
        return match self.rows.get(x as usize) {
            Some(row) => row.get(y as usize),
            None => None,
        };
    }

    fn count_seats(&self, tile_type: Tile) -> i32 {
        return self.rows.iter().fold(0, |n, row| {
            return n + row.iter().fold(0, |n, tile| {
                return n + if tile_type == *tile { 1 } else { 0 };
            });
        });
    }

    fn count_neighbours(&self, x: i32, y: i32) -> i32 {
        let mut neighbours = 0;
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for (d_x, d_y) in directions.iter() {
            match self.get_tile(x + d_x, y + d_y) {
                Some(tile) => match tile {
                    Tile::Taken => {
                        neighbours += 1;
                    }
                    _ => {}
                },
                None => {}
            };
        }

        return neighbours;
    }

    fn count_visible_neighbours(&self, x: i32, y: i32) -> i32 {
        let mut neighbours = 0;

        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for (d_x, d_y) in directions.iter() {
            for distance in 1.. {
                match self.get_tile(x + d_x * distance, y + d_y * distance) {
                    None => {
                        break;
                    }
                    Some(tile) => match tile {
                        Tile::Taken => {
                            neighbours += 1;
                            break;
                        }
                        Tile::Empty => {
                            break;
                        }
                        _ => {}
                    },
                };
            }
        }

        return neighbours;
    }

    fn is_crowded(&self) -> bool {
        for x in 0..self.width {
            for y in 0..self.height {
                if *self.get_tile(x, y).unwrap() == Tile::Taken && self.count_neighbours(x, y) >= 4
                {
                    return true;
                }
            }
        }

        return false;
    }

    fn has_free_seats(&self) -> bool {
        for x in 0..self.width {
            for y in 0..self.height {
                if *self.get_tile(x, y).unwrap() == Tile::Empty && self.count_neighbours(x, y) == 0
                {
                    return true;
                }
            }
        }

        return false;
    }

    fn is_crowded_2(&self) -> bool {
        for x in 0..self.width {
            for y in 0..self.height {
                if *self.get_tile(x, y).unwrap() == Tile::Taken
                    && self.count_visible_neighbours(x, y) >= 5
                {
                    return true;
                }
            }
        }

        return false;
    }

    fn has_free_seats_2(&self) -> bool {
        for x in 0..self.width {
            for y in 0..self.height {
                if *self.get_tile(x, y).unwrap() == Tile::Empty
                    && self.count_visible_neighbours(x, y) == 0
                {
                    return true;
                }
            }
        }

        return false;
    }

    fn remove_seats(&mut self) {
        self.rows = (0..self.width)
            .map(|x| {
                (0..self.height)
                    .map(|y| match *self.get_tile(x, y).unwrap() {
                        Tile::Taken => {
                            if self.count_neighbours(x, y) >= 4 {
                                Tile::Empty
                            } else {
                                Tile::Taken
                            }
                        }
                        Tile::Floor => Tile::Floor,
                        Tile::Empty => Tile::Empty,
                    })
                    .collect()
            })
            .collect();
    }

    fn remove_seats_2(&mut self) {
        self.rows = (0..self.width)
            .map(|x| {
                (0..self.height)
                    .map(|y| match *self.get_tile(x, y).unwrap() {
                        Tile::Taken => {
                            if self.count_visible_neighbours(x, y) >= 5 {
                                Tile::Empty
                            } else {
                                Tile::Taken
                            }
                        }
                        Tile::Floor => Tile::Floor,
                        Tile::Empty => Tile::Empty,
                    })
                    .collect()
            })
            .collect();
    }

    fn add_seats(&mut self) {
        self.rows = (0..self.width)
            .map(|x| {
                (0..self.height)
                    .map(|y| match *self.get_tile(x, y).unwrap() {
                        Tile::Empty => {
                            if self.count_neighbours(x, y) == 0 {
                                Tile::Taken
                            } else {
                                Tile::Empty
                            }
                        }
                        Tile::Floor => Tile::Floor,
                        Tile::Taken => Tile::Taken,
                    })
                    .collect()
            })
            .collect();
    }

    fn add_seats_2(&mut self) {
        self.rows = (0..self.width)
            .map(|x| {
                (0..self.height)
                    .map(|y| match *self.get_tile(x, y).unwrap() {
                        Tile::Empty => {
                            if self.count_visible_neighbours(x, y) == 0 {
                                Tile::Taken
                            } else {
                                Tile::Empty
                            }
                        }
                        Tile::Floor => Tile::Floor,
                        Tile::Taken => Tile::Taken,
                    })
                    .collect()
            })
            .collect();
    }
}
