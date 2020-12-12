enum Instruction {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Left(isize),
    Right(isize),
    Forward(isize),
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    West,
    East,
    South,
}

impl Direction {
    fn left(self, degrees: isize) -> Direction {
        let mut result = self;
        let mut degrees = degrees;
        while degrees >= 90 {
            result = match result {
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
            };
            degrees -= 90
        }
        result
    }
    fn right(self, degrees: isize) -> Direction {
        let mut result = self;
        let mut degrees = degrees;
        while degrees >= 90 {
            result = match result {
                Direction::North => Direction::East,
                Direction::West => Direction::North,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
            };
            degrees -= 90
        }
        result
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let (op, number) = line.split_at(1);
            let value = number.parse().unwrap();
            match op {
                "N" => Instruction::North(value),
                "S" => Instruction::South(value),
                "E" => Instruction::East(value),
                "W" => Instruction::West(value),
                "L" => Instruction::Left(value),
                "R" => Instruction::Right(value),
                "F" => Instruction::Forward(value),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("inputs/day12/input").unwrap();
    let instructions = parse(&input);

    let mut direction = Direction::East;
    let mut position = (0isize, 0isize);
    for instr in instructions.iter() {
        let (new_dir, new_pos) = match instr {
            Instruction::North(v) => (direction, (position.0, position.1 + *v)),
            Instruction::South(v) => (direction, (position.0, position.1 - *v)),
            Instruction::East(v) => (direction, (position.0 + *v, position.1)),
            Instruction::West(v) => (direction, (position.0 - *v, position.1)),
            Instruction::Left(v) => (direction.left(*v), position),
            Instruction::Right(v) => (direction.right(*v), position),
            Instruction::Forward(v) => (
                direction,
                match direction {
                    Direction::North => (position.0, position.1 + *v),
                    Direction::West => (position.0 - *v, position.1),
                    Direction::East => (position.0 + *v, position.1),
                    Direction::South => (position.0, position.1 - *v),
                },
            ),
        };
        direction = new_dir;
        position = new_pos;
    }

    println!("Solution 1: {}", position.0.abs() + position.1.abs());

    let mut waypoint = (10isize, 1isize);
    let mut position = (0isize, 0isize);
    for instr in instructions.iter() {
        let (new_pos, new_waypoint) = match instr {
            Instruction::North(v) => (position, (waypoint.0, waypoint.1 + *v)),
            Instruction::South(v) => (position, (waypoint.0, waypoint.1 - *v)),
            Instruction::East(v) => (position, (waypoint.0 + *v, waypoint.1)),
            Instruction::West(v) => (position, (waypoint.0 - *v, waypoint.1)),
            Instruction::Left(v) => (position, rotate_waypoint_left(waypoint, *v)),
            Instruction::Right(v) => (position, rotate_waypoint_right(waypoint, *v)),
            Instruction::Forward(v) => (
                (position.0 + waypoint.0 * *v, position.1 + waypoint.1 * *v),
                waypoint,
            ),
        };
        position = new_pos;
        waypoint = new_waypoint;
    }

    println!("Solution 2: {}", position.0.abs() + position.1.abs());

    fn rotate_waypoint_left(waypoint: (isize, isize), degrees: isize) -> (isize, isize) {
        let mut waypoint = waypoint;
        let mut degrees = degrees;
        while degrees >= 90 {
            waypoint = (-waypoint.1, waypoint.0);
            degrees -= 90;
        }
        waypoint
    }

    fn rotate_waypoint_right(waypoint: (isize, isize), degrees: isize) -> (isize, isize) {
        let mut waypoint = waypoint;
        let mut degrees = degrees;
        while degrees >= 90 {
            waypoint = (waypoint.1, -waypoint.0);
            degrees -= 90;
        }
        waypoint
    }
}
