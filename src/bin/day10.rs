fn main() {
    let input = std::fs::read_to_string("inputs/day10/input").unwrap();
    let mut numbers: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    numbers.sort_unstable();

    let mut number_of_jumps = [0, 0, 1];
    let mut jolts = 0;
    for rating in numbers.iter() {
        let jump = rating - jolts;
        number_of_jumps[jump - 1] += 1;
        jolts = *rating;
    }

    println!("Solution 1: {}", number_of_jumps[0] * number_of_jumps[2]);

    numbers.insert(0, 0);
    let mut ways_to_get_there = vec![0usize; numbers.len()];
    ways_to_get_there[0] = 1;
    for i in 0..numbers.len() {
        let ways_to_get_here = ways_to_get_there[i];
        let rating = numbers[i];
        for j in 1..=3 {
            if i + j >= numbers.len() || numbers[i + j] - rating > 3 {
                break;
            }
            ways_to_get_there[i + j] += ways_to_get_here;
        }
    }

    println!("Solution 2: {}", ways_to_get_there.last().unwrap());
}
