use adventofcode2020::regex;
use std::collections::HashMap;
use std::ops::RangeInclusive;

struct Rule {
    field: String,
    a: RangeInclusive<usize>,
    b: RangeInclusive<usize>,
}

impl Rule {
    fn contains(&self, value: usize) -> bool {
        self.a.contains(&value) || self.b.contains(&value)
    }
}

#[derive(Default)]
struct Input {
    rules: Vec<Rule>,
    my_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

fn parse(input: &str) -> Input {
    let mut lines = input.lines();
    let mut result = Input::default();

    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }

        let captures = regex!("^(.*): (\\d+)-(\\d+) or (\\d+)-(\\d+)$")
            .captures(line)
            .unwrap();
        result.rules.push(Rule {
            field: (&captures[1]).to_owned(),
            a: (&captures[2]).parse().unwrap()..=(&captures[3]).parse().unwrap(),
            b: (&captures[4]).parse().unwrap()..=(&captures[5]).parse().unwrap(),
        });
    }

    lines.next().unwrap();
    result.my_ticket = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    lines.next().unwrap();

    lines.next().unwrap();
    result.nearby_tickets = lines
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    result
}

fn main() {
    let input = std::fs::read_to_string("inputs/day16/input").unwrap();
    let input = parse(&input);

    let error: usize = input
        .nearby_tickets
        .iter()
        .flatten()
        .filter(|field| !input.rules.iter().any(|rule| rule.contains(**field)))
        .sum();

    println!("Solution 1: {}", error);

    let valid_tickets: Vec<&[usize]> = input
        .nearby_tickets
        .iter()
        .filter(|fields| {
            !fields
                .iter()
                .any(|field| !input.rules.iter().any(|rule| rule.contains(*field)))
        })
        .map(AsRef::as_ref)
        .collect();

    let num_fields = valid_tickets[0].len();
    let rules: Vec<(usize, &Rule)> = input.rules.iter().enumerate().collect();

    let rules_per_field: Vec<Vec<usize>> = (0..num_fields)
        .map(|field_idx| {
            rules
                .iter()
                .filter_map(|(rule_idx, rule)| {
                    if valid_tickets
                        .iter()
                        .map(move |fields| fields[field_idx])
                        .all(|v| rule.contains(v))
                    {
                        Some(*rule_idx)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();

    let mut rule_to_field: HashMap<usize, usize> = Default::default();
    while rule_to_field.len() != input.rules.len() {
        for (field_idx, rules) in rules_per_field.iter().enumerate() {
            let mut rules = rules
                .iter()
                .filter(|rule_idx| !rule_to_field.contains_key(rule_idx));
            if let Some(rule_idx) = rules.next() {
                if rules.next().is_none() {
                    rule_to_field.insert(*rule_idx, field_idx);
                }
            }
        }
    }

    let product: usize = input
        .rules
        .iter()
        .enumerate()
        .filter_map(|(rule_idx, rule)| {
            if rule.field.starts_with("departure") {
                Some(input.my_ticket[rule_to_field[&rule_idx]])
            } else {
                None
            }
        })
        .product();

    println!("Solution 2: {}", product);
}
