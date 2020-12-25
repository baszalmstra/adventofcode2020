use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn move_to(self, direction: Direction) -> Self {
        match direction {
            Direction::East => Point {
                x: self.x - 1,
                y: self.y,
            },
            Direction::SouthEast => Point {
                x: if self.y % 2 == 0 { self.x - 1 } else { self.x },
                y: self.y + 1,
            },
            Direction::SouthWest => Point {
                x: if self.y % 2 == 0 { self.x } else { self.x + 1 },
                y: self.y + 1,
            },
            Direction::West => Point {
                x: self.x + 1,
                y: self.y,
            },
            Direction::NorthWest => Point {
                x: if self.y % 2 == 0 { self.x } else { self.x + 1 },
                y: self.y - 1,
            },
            Direction::NorthEast => Point {
                x: if self.y % 2 == 0 { self.x - 1 } else { self.x },
                y: self.y - 1,
            },
        }
    }

    fn neighbours(self) -> [Self; 6] {
        [
            self.move_to(Direction::East),
            self.move_to(Direction::SouthEast),
            self.move_to(Direction::SouthWest),
            self.move_to(Direction::West),
            self.move_to(Direction::NorthWest),
            self.move_to(Direction::NorthEast),
        ]
    }
}

enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|l| {
            let mut chars = l.chars();
            let mut directions = Vec::new();
            while let Some(c) = chars.next() {
                directions.push(match c {
                    'e' => Direction::East,
                    's' => match chars.next().unwrap() {
                        'e' => Direction::SouthEast,
                        'w' => Direction::SouthWest,
                        _ => unreachable!(),
                    },
                    'w' => Direction::West,
                    'n' => match chars.next().unwrap() {
                        'e' => Direction::NorthEast,
                        'w' => Direction::NorthWest,
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                })
            }
            directions
        })
        .map(|directions| {
            directions
                .into_iter()
                .fold(Point::default(), Point::move_to)
        })
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("inputs/day24/input").unwrap();
    let input = parse(&input);

    let mut black_tiles = HashSet::new();
    for point in input.iter() {
        if black_tiles.contains(point) {
            black_tiles.remove(point);
        } else {
            black_tiles.insert(*point);
        }
    }
    println!("Solution 1: {}", black_tiles.len());

    for _day in 0..100 {
        let mut number_of_black_neighbours = HashMap::new();
        for black_point in black_tiles.iter() {
            for neighbor in &black_point.neighbours() {
                let count = number_of_black_neighbours.entry(*neighbor).or_insert(0);
                *count += 1;
            }
        }

        let mut new_black_tiles = HashSet::new();
        for point in black_tiles.iter().chain(number_of_black_neighbours.keys()) {
            let is_black = black_tiles.contains(point);
            let black_neighbours = number_of_black_neighbours.get(point).copied().unwrap_or(0);
            if (is_black && !(black_neighbours == 0 || black_neighbours > 2))
                || (!is_black && black_neighbours == 2)
            {
                new_black_tiles.insert(*point);
            }
        }

        black_tiles = new_black_tiles;
    }
    println!("Solution 2: {}", black_tiles.len());
}
