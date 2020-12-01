use std::iter::FromIterator;
use std::str::FromStr;

/// Puzzle input
struct Input(pub Vec<isize>);

// Conversion from string to puzzle input
impl FromStr for Input {
    type Err = <isize as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Result::from_iter(s.lines().map(|line| line.parse())).map(Input)
    }
}

/// Main function that loads the input for this day and computes the solution
fn main() {
    let input = std::fs::read_to_string("inputs/day1/input")
        .unwrap()
        .parse()
        .unwrap();
    println!("solution 1 = {}", solve(&input, 2).unwrap());
    println!("solution 2 = {}", solve(&input, 3).unwrap());
}

/// Given the puzzle input computes the solution
fn solve(input: &Input, count: isize) -> Option<isize> {
    fn solve_n(input: &[isize], count: isize, sum: isize) -> Option<isize> {
        if count == 1 {
            if input.contains(&sum) {
                return Some(sum);
            }
        } else {
            for i in 0..input.len() {
                let a = input[i];
                if let Some(b) = solve_n(&input[i + 1..input.len()], count - 1, sum - a) {
                    return Some(a * b);
                }
            }
        }
        None
    }

    solve_n(&input.0, count, 2020)
}

#[cfg(test)]
mod test {
    use crate::solve;

    const EXAMPLE: &str = "1721\n979\n366\n299\n675\n1456";

    #[test]
    fn example() {
        assert_eq!(solve(&EXAMPLE.parse().unwrap(), 2), Some(514579));
    }

    #[test]
    fn example2() {
        assert_eq!(solve(&EXAMPLE.parse().unwrap(), 3), Some(241861950));
    }
}
