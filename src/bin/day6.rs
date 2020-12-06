use std::collections::HashSet;

#[derive(Default)]
struct Group {
    answers_per_person: Vec<HashSet<char>>,
}

fn parse(input: &str) -> Vec<Group> {
    let mut result = Vec::new();
    result.push(Group::default());
    for line in input.lines() {
        if line.is_empty() {
            result.push(Group::default());
            continue;
        }

        let group = result.last_mut().unwrap();
        group.answers_per_person.push(line.chars().collect());
    }
    result
}

fn main() {
    let input = std::fs::read_to_string("inputs/day6/input").unwrap();
    let groups = parse(&input);

    let answer_count: usize = groups
        .iter()
        .map(|group| {
            group
                .answers_per_person
                .iter()
                .fold(HashSet::default(), |a, b| a.union(b).copied().collect())
        })
        .map(|answers| answers.len())
        .sum();

    println!("Solution 1: {}", answer_count);

    let answer_count: usize = groups
        .iter()
        .filter_map(|group| {
            let mut iter = group.answers_per_person.iter();
            let first = iter.next().cloned();
            first.map(|a| iter.fold(a, |a, b| a.intersection(b).copied().collect()))
        })
        .map(|answers| answers.len())
        .sum();

    println!("Solution 2: {}", answer_count);
}
