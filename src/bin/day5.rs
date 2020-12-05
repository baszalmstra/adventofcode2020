fn main() {
    let input = std::fs::read_to_string("inputs/day5/input").unwrap();
    let passports = input.lines().map(boarding_pass_to_id).collect::<Vec<_>>();

    let min = passports.iter().min().unwrap();
    let max = passports.iter().max().unwrap();

    println!("Solution 1: {}", max);

    let mut entry_exists = vec![false; max - min + 1];
    for idx in passports.iter() {
        entry_exists[idx - min] = true;
    }
    let mut entry = entry_exists.into_iter().enumerate().filter(|p| !p.1);
    println!("Solution 2: {}", entry.next().unwrap().0 + min);
    assert!(entry.next().is_none());
}

#[derive(Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

fn boarding_pass_to_id(boarding_pass: &str) -> usize {
    let row = binary_space_id(
        0,
        127,
        boarding_pass.chars().take(7).map(|c| match c {
            'F' => Direction::Left,
            'B' => Direction::Right,
            c => unreachable!("char: {}", c),
        }),
    );
    let column = binary_space_id(
        0,
        7,
        boarding_pass.chars().skip(7).map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            c => unreachable!("char: {}", c),
        }),
    );
    row * 8 + column
}

fn binary_space_id(
    min: usize,
    max: usize,
    mut directions: impl Iterator<Item = Direction>,
) -> usize {
    if max == min {
        max
    } else if directions.next().unwrap() == Direction::Left {
        binary_space_id(min, min + (max - min) / 2, directions)
    } else {
        binary_space_id(min + (max - min) / 2 + 1, max, directions)
    }
}
