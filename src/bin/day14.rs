use adventofcode2020::regex;
use std::collections::HashMap;

struct Mask {
    and_mask: usize,
    or_mask: usize,
    floating_bits: Vec<usize>,
    floating_bit_mask: usize,
}

enum Operation {
    Mask(Mask),
    Assignment { address: usize, value: usize },
}

fn parse(input: &str) -> Vec<Operation> {
    input
        .lines()
        .map(|line| {
            if let Some(captures) = regex!("mask = ([X01]{36})").captures(line) {
                let mut and_mask = !0usize;
                let mut or_mask = 0;
                let mut floating_bits = Vec::new();
                let mut floating_bit_mask = !0usize;
                for (i, c) in captures[1].chars().rev().enumerate() {
                    match c {
                        'X' => {
                            floating_bits.push(i);
                            floating_bit_mask &= !(1 << i);
                        }
                        '0' => {
                            and_mask &= !(1 << i);
                        }
                        '1' => {
                            or_mask |= 1 << i;
                        }
                        _ => unreachable!(),
                    }
                }
                Operation::Mask(Mask {
                    and_mask,
                    or_mask,
                    floating_bits,
                    floating_bit_mask,
                })
            } else if let Some(captures) = regex!("mem\\[(\\d+)\\] = (\\d+)").captures(line) {
                Operation::Assignment {
                    address: captures[1].parse().unwrap(),
                    value: captures[2].parse().unwrap(),
                }
            } else {
                unreachable!("{}", line)
            }
        })
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("inputs/day14/input").unwrap();
    let ops = parse(&input);

    let mut mask = None;
    let mut values = HashMap::<usize, usize>::new();
    for op in ops.iter() {
        match op {
            Operation::Mask(m) => {
                mask = Some(m);
            }
            Operation::Assignment { address, value } => {
                let mask = mask.unwrap();
                let new_value = *value & mask.and_mask | mask.or_mask;
                values.insert(*address, new_value);
            }
        }
    }

    println!("Solution 1: {}", values.values().sum::<usize>());

    let mut mask = None;
    let mut values = HashMap::<usize, usize>::new();
    for op in ops.iter() {
        match op {
            Operation::Mask(m) => {
                mask = Some(m);
            }
            Operation::Assignment { address, value } => {
                let mask = mask.unwrap();

                let mut addresses = Vec::new();
                fn build_addresses(
                    floating_bits: &[usize],
                    start_address: usize,
                    addresses: &mut Vec<usize>,
                ) {
                    let or_address = start_address | (1 << floating_bits[0]);
                    let and_address = start_address & !(1 << floating_bits[0]);
                    for address in [or_address, and_address].iter() {
                        if floating_bits.len() > 1 {
                            build_addresses(&floating_bits[1..], *address, addresses);
                        } else {
                            addresses.push(*address);
                        }
                    }
                }

                let masked_address = (*address & mask.floating_bit_mask) | mask.or_mask;
                build_addresses(&mask.floating_bits, masked_address, &mut addresses);

                for addr in addresses {
                    values.insert(addr, *value);
                }
            }
        }
    }

    println!("Solution 2: {}", values.values().sum::<usize>());
}
