fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part1(numbers: &[usize], preamble: usize) -> usize {
    for i in preamble..numbers.len() {
        let number = numbers[i];

        let mut found = false;
        for a in i-preamble..i {
            for b in a..i {
                if numbers[a] + numbers[b] == number {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }

        if !found {
            return number;
        }
    }

    unreachable!();
}

fn main() {
    let input = std::fs::read_to_string("inputs/day9/input").unwrap();
    let numbers = parse(&input);

    let preamble = 25;
    let invalid_number = part1(&numbers, preamble);
    println!("Solution 1: {}", invalid_number);

    for start in 0..numbers.len() {
        let mut acc = numbers[start];
        let mut end = start+1;
        let result = loop {
            acc += numbers[end];
            if acc == invalid_number {
                break Some(end);
            }
            if acc > invalid_number {
                break None;
            };
            end += 1;
        };
        if let Some(result) = result {
            let smallest = numbers[start..=result].iter().min().unwrap();
            let largest = numbers[start..=result].iter().max().unwrap();
            println!("Solution 2: {}", smallest + largest);
            break;
        }
    }
}
