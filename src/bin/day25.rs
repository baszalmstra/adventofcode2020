fn parse(input: &str) -> (usize, usize) {
    let mut lines = input.lines();
    (
        lines.next().unwrap().parse().unwrap(),
        lines.next().unwrap().parse().unwrap(),
    )
}

fn handshake(subject_number: usize, loop_size: usize) -> usize {
    let mut result = 1;
    for _ in 0..loop_size {
        result = (result * subject_number) % 20201227;
    }
    result
}

fn find_loop_size(subject_number: usize, encryption_key: usize) -> usize {
    let mut loop_size = 0;
    let mut result = 1;
    loop {
        if encryption_key == result {
            return loop_size;
        }

        loop_size += 1;
        result = (result * subject_number) % 20201227;
    }
}

fn main() {
    let input = std::fs::read_to_string("inputs/day25/input").unwrap();
    let (card_public_key, door_public_key) = parse(&input);

    let card_loop_size = find_loop_size(7, card_public_key);
    let door_loop_size = find_loop_size(7, door_public_key);

    let encryption_key = handshake(card_public_key, door_loop_size);
    assert_eq!(handshake(door_public_key, card_loop_size), encryption_key);

    println!("Solution 1: {}", encryption_key);
}
