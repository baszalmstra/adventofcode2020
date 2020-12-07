use adventofcode2020::regex;
use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;

#[derive(Default)]
struct BagRule {
    bag: usize,
    count: usize,
    contains: usize,
}

#[derive(Default)]
struct BagGraph {
    bag_to_id: HashMap<String, usize>,
    id_to_bag: Vec<String>,
    id_to_contains_rules: Vec<Vec<usize>>,
    id_to_contained_rules: Vec<Vec<usize>>,
    rules: Vec<BagRule>,
}

impl BagGraph {
    fn allocate_bag_id(&mut self, name: &str) -> usize {
        let id = self.bag_to_id.len();
        self.bag_to_id.insert(name.to_owned(), id);
        self.id_to_bag.push(name.to_owned());
        self.id_to_contains_rules.push(Default::default());
        self.id_to_contained_rules.push(Default::default());
        id
    }
}

#[derive(Default)]
struct RawBagRule {
    bag: String,
    contains: Vec<(String, usize)>,
}

fn parse_rule(input: &str) -> RawBagRule {
    let captures = regex!("^(.*) bags contain (.*).$").captures(input).unwrap();
    let bag = captures[1].to_owned();
    let contains = regex!("(\\d) (.*?) bag[s]?")
        .captures_iter(&captures[2])
        .map(|c| (c[2].to_owned(), c[1].parse().unwrap()))
        .collect();
    RawBagRule { bag, contains }
}

fn parse(input: &str) -> Vec<RawBagRule> {
    input.lines().map(parse_rule).collect()
}

fn main() {
    let input = std::fs::read_to_string("inputs/day7/input").unwrap();
    let rules = parse(&input);

    let mut graph = BagGraph::default();

    // Insert all nodes
    let my_bag = graph.allocate_bag_id("shiny gold");
    for rule in rules.iter() {
        if !graph.bag_to_id.contains_key(&rule.bag) {
            graph.allocate_bag_id(&rule.bag);
        }
    }

    // Create all edges in the graph
    for rule in rules {
        let bag = *graph.bag_to_id.get(&rule.bag).unwrap();
        for (contains, count) in rule.contains {
            let contained_bag = *graph.bag_to_id.get(&contains).unwrap();
            let rule_id = graph.rules.len();
            graph.rules.push(BagRule {
                bag,
                count,
                contains: contained_bag,
            });
            graph.id_to_contained_rules[contained_bag].push(rule_id);
            graph.id_to_contains_rules[bag].push(rule_id);
        }
    }

    let mut contains_my_bag = vec![false; graph.id_to_bag.len()];
    let mut stack = VecDeque::from_iter(graph.id_to_contained_rules[my_bag].iter().copied());

    while let Some(elem) = stack.pop_front() {
        let rule = &graph.rules[elem];
        contains_my_bag[rule.bag] = true;
        for rule_id in graph.id_to_contained_rules[rule.bag].iter() {
            stack.push_back(*rule_id)
        }
    }

    println!(
        "Solution 1: {}",
        contains_my_bag
            .into_iter()
            .filter(|can_contain| *can_contain)
            .count()
    );

    let total_count =
        compute_contained_bag_count(&graph, my_bag, &mut vec![None; graph.id_to_bag.len()]);
    fn compute_contained_bag_count(
        graph: &BagGraph,
        bag: usize,
        bag_count_cache: &mut Vec<Option<usize>>,
    ) -> usize {
        if let Some(bag_count) = bag_count_cache[bag] {
            return bag_count;
        }

        let mut count = 0;
        for rule_id in graph.id_to_contains_rules[bag].iter() {
            let rule = &graph.rules[*rule_id];
            count += (1 + compute_contained_bag_count(graph, rule.contains, bag_count_cache))
                * rule.count;
        }
        bag_count_cache[bag] = Some(count);
        count
    }

    println!("Solution 2: {}", total_count);
}
