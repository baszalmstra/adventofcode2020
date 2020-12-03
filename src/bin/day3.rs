fn main() {
    let input: Vec<Vec<_>> = std::fs::read_to_string("inputs/day3/input")
        .unwrap()
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let directions: Vec<_> = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|dir| count_trees(&input, *dir))
        .collect();

    println!("solution 1: {}", directions[1]);
    println!("solution 2: {}", directions.iter().product::<usize>());
}

fn count_trees(input: &[Vec<bool>], dir: (usize, usize)) -> usize {
    let width = input[0].len();
    let height = input.len();
    let mut pos: (usize, usize) = (0, 0);
    let mut count = 0;
    loop {
        let new_pos: (usize, usize) = ((pos.0 + dir.0) % width, pos.1 + dir.1);
        if new_pos.1 >= height {
            return count;
        }
        pos = new_pos;
        if input[pos.1][pos.0] {
            count += 1;
        }
    }
}
