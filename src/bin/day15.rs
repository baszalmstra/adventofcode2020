fn count(until_turn: u32, starting_numbers: &[u32]) -> u32 {
    let mut last_spoken_in_turn: Vec<Option<u32>> = vec![None; until_turn as usize];
    let mut last_number_spoken = None;
    for turn in 0..until_turn {
        let number = if turn < starting_numbers.len() as u32 {
            starting_numbers[turn as usize]
        } else {
            let last_number_spoken = last_number_spoken.unwrap();
            last_spoken_in_turn[last_number_spoken as usize]
                .map(|n| turn - n - 1)
                .unwrap_or(0)
        };

        if let Some(last_number_spoken) = last_number_spoken {
            last_spoken_in_turn[last_number_spoken as usize] = Some(turn - 1);
        }
        last_number_spoken = Some(number);
    }

    last_number_spoken.unwrap()
}

fn main() {
    let input = std::fs::read_to_string("inputs/day15/input").unwrap();
    let starting_numbers: Vec<u32> = input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    println!("Solution 1: {}", count(2020, &starting_numbers));
    println!("Solution 2: {}", count(30000000, &starting_numbers));
}
