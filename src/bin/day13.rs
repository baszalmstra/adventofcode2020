fn parse(input: &str) -> (isize, Vec<(isize, isize)>) {
    let mut lines = input.lines();
    (
        lines.next().unwrap().parse().unwrap(),
        lines
            .next()
            .unwrap()
            .split(',')
            .enumerate()
            .filter_map(|(i, id)| {
                if id == "x" {
                    None
                } else {
                    Some((i as isize, id.parse().unwrap()))
                }
            })
            .collect(),
    )
}

fn earliest_time_for_bus(earliest_time: isize, id: isize) -> isize {
    (id - (earliest_time % id)) % id
}

fn inv_mod(x: isize, p: isize) -> isize {
    (0..p - 2).fold(1, |o, _| (o * x) % p)
}

fn main() {
    let input = std::fs::read_to_string("inputs/day13/input").unwrap();
    let (earliest_time, bus_ids) = parse(&input);

    let (wait_time, earliest_bus) = bus_ids
        .iter()
        .min_by_key(|(_, id)| earliest_time_for_bus(earliest_time, *id))
        .unwrap();

    println!("Solution 1: {}", *earliest_bus * *wait_time);

    let prod: isize = bus_ids.iter().map(|(_, b)| b).product();

    let result2 = bus_ids
        .iter()
        .map(|&(a, b)| -a * (prod / b) * inv_mod(prod / b, b))
        .sum::<isize>()
        .rem_euclid(prod);

    println!("Solution 2: {}", result2);
}
