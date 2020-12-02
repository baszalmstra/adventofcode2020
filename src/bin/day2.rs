use std::ops::RangeInclusive;

struct PasswordPolicy {
    range: RangeInclusive<usize>,
    char: char,
}

fn parse(line: &str) -> (PasswordPolicy, Vec<char>) {
    let mut identifiers = line
        .split(['-', ':', ' '].as_ref())
        .filter(|s| !s.is_empty());
    (
        PasswordPolicy {
            range: RangeInclusive::new(
                identifiers.next().unwrap().parse().unwrap(),
                identifiers.next().unwrap().parse().unwrap(),
            ),
            char: identifiers.next().unwrap().parse().unwrap(),
        },
        identifiers.next().unwrap().chars().collect(),
    )
}

fn is_valid_password(policy: &PasswordPolicy, password: &[char]) -> bool {
    policy
        .range
        .contains(&password.iter().filter(|c| **c == policy.char).count())
}

fn is_valid_password2(policy: &PasswordPolicy, password: &[char]) -> bool {
    let first = password[*policy.range.start() - 1] == policy.char;
    let second = password[*policy.range.end() - 1] == policy.char;
    first ^ second
}

fn main() {
    let content = std::fs::read_to_string("inputs/day2/input").unwrap();
    let input: Vec<_> = content.lines().map(parse).collect();

    let valid_passwords = input
        .iter()
        .filter(|(policy, password)| is_valid_password(policy, password))
        .count();
    let valid_passwords2 = input
        .iter()
        .filter(|(policy, password)| is_valid_password2(policy, password))
        .count();

    println!("solution 1: {}", valid_passwords);
    println!("solution 2: {}", valid_passwords2);
}
