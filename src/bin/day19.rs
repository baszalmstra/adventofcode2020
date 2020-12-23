use std::collections::HashMap;

#[derive(Debug)]
enum Rule {
    Sequence(Vec<usize>),
    Or(Vec<Rule>),
    Char(char),
}

fn parse_rule_atom(line: &str) -> Rule {
    let line = line.trim();
    if line.contains('|') {
        let or_split = line.split('|').peekable();
        Rule::Or(or_split.map(parse_rule_atom).collect())
    } else if line.starts_with('"') {
        Rule::Char(line.chars().nth(1).unwrap())
    } else {
        Rule::Sequence(
            line.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
        )
    }
}

fn parse_rule(line: &str) -> (usize, Rule) {
    let mut split = line.split(':');
    let rule_idx = split.next().unwrap().parse().unwrap();
    (rule_idx, parse_rule_atom(split.next().unwrap()))
}

fn parse(input: &str) -> (HashMap<usize, Rule>, Vec<&str>) {
    let mut lines = input.lines();

    let mut rules = HashMap::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (n, rule) = parse_rule(line);
        rules.insert(n, rule);
    }

    (rules, lines.collect())
}

fn matches<'a>(
    chars: &'a [char],
    rule: &Rule,
    rules: &HashMap<usize, Rule>,
) -> Option<Vec<&'a [char]>> {
    match rule {
        Rule::Char(c) => {
            if *chars.first()? == *c {
                Some(vec![&chars[1..]])
            } else {
                None
            }
        }
        Rule::Or(or_sequence) => {
            let mut results = or_sequence
                .iter()
                .filter_map(|option| matches(chars, option, rules))
                .peekable();
            if results.peek().is_some() {
                Some(results.flatten().collect())
            } else {
                None
            }
        }
        Rule::Sequence(sequence) => {
            let mut results = vec![chars];
            for entry in sequence {
                let mut new_results = results
                    .into_iter()
                    .filter_map(|previous_result| matches(&previous_result, &rules[entry], rules))
                    .peekable();
                if new_results.peek().is_some() {
                    results = new_results.flatten().collect();
                } else {
                    return None;
                }
            }
            Some(results)
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("inputs/day19/input").unwrap();
    let (rules, input) = parse(&input);

    println!(
        "Solution 1: {}",
        input
            .iter()
            .filter(|message| {
                matches(&message.chars().collect::<Vec<_>>(), &rules[&0], &rules)
                    .map(|results| results.iter().any(|r| r.is_empty()))
                    .unwrap_or(false)
            })
            .count()
    );

    let input = std::fs::read_to_string("inputs/day19/input_updated").unwrap();
    let (rules, input) = parse(&input);

    println!(
        "Solution 2: {}",
        input
            .iter()
            .filter(|message| {
                matches(&message.chars().collect::<Vec<_>>(), &rules[&0], &rules)
                    .map(|results| results.iter().any(|r| r.is_empty()))
                    .unwrap_or(false)
            })
            .count()
    );
}
