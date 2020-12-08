#[derive(Debug, Copy, Clone)]
enum Op {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

#[derive(Default)]
struct State {
    cursor: isize,
    accumulator: isize,
}

impl Op {
    fn execute(&self, state: &mut State) {
        match self {
            Op::Acc(value) => {
                state.accumulator += *value;
                state.cursor += 1;
            }
            Op::Jmp(offset) => {
                state.cursor += *offset;
            }
            Op::Nop(_) => {
                state.cursor += 1;
            }
        };
    }

    fn flip_corruption(&self) -> Op {
        match self {
            Op::Jmp(value) => Op::Nop(*value),
            Op::Nop(value) => Op::Jmp(*value),
            a => *a,
        }
    }
}

fn parse(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            match split.next() {
                Some("nop") => Op::Nop(split.next().unwrap().parse().unwrap()),
                Some("acc") => Op::Acc(split.next().unwrap().parse().unwrap()),
                Some("jmp") => Op::Jmp(split.next().unwrap().parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect()
}

enum RunResult {
    Cycle(isize),
    Completion(isize),
}

fn run(ops: &[Op]) -> RunResult {
    let mut state = State::default();
    let mut instruction_executed = vec![false; ops.len()];
    loop {
        if instruction_executed[state.cursor as usize] {
            break RunResult::Cycle(state.accumulator);
        }
        instruction_executed[state.cursor as usize] = true;
        ops[state.cursor as usize].execute(&mut state);
        if state.cursor as usize == ops.len() {
            break RunResult::Completion(state.accumulator);
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("inputs/day8/input").unwrap();
    let mut ops = parse(&input);

    if let RunResult::Cycle(acc) = run(&ops) {
        println!("Solution 1: {}", acc);
    }

    for i in 0..ops.len() {
        if let Op::Acc(_) = ops[i] {
            continue;
        }
        ops[i] = ops[i].flip_corruption();
        if let RunResult::Completion(acc) = run(&ops) {
            println!("Solution 2: {}", acc)
        }
        ops[i] = ops[i].flip_corruption();
    }
}
