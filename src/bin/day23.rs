struct CupIterator<'a> {
    cup_to_next_cup: &'a [usize],
    current: usize,
    count: usize,
}

impl<'a> CupIterator<'a> {
    pub fn new(cup_to_next_cup: &'a [usize], first: usize) -> Self {
        Self {
            cup_to_next_cup,
            current: first,
            count: 0,
        }
    }
}

impl<'a> Iterator for CupIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.cup_to_next_cup.len() {
            None
        } else {
            let current = self.current;
            self.current = self.cup_to_next_cup[current];
            self.count += 1;
            Some(current)
        }
    }
}

fn cups_to_string(cups: &[usize]) -> String {
    CupIterator::new(&cups, 0)
        .map(|v| v + 1)
        .skip(1)
        .take(8)
        .map(|n| n.to_string())
        .collect()
}

fn take_turns(cup_to_next_cup: &mut [usize], mut current_cup: usize, turns: usize) {
    for _ in 0..turns {
        let next_cup = cup_to_next_cup[current_cup];

        let mut picked_up_iter = CupIterator::new(&cup_to_next_cup, next_cup);
        let picked_up = [
            picked_up_iter.next().unwrap(),
            picked_up_iter.next().unwrap(),
            picked_up_iter.next().unwrap(),
        ];

        cup_to_next_cup[current_cup] = cup_to_next_cup[picked_up[2]];

        let mut destination_cup = if current_cup > 0 {
            current_cup - 1
        } else {
            cup_to_next_cup.len() - 1
        };
        while picked_up.contains(&destination_cup) {
            if destination_cup > 0 {
                destination_cup -= 1;
            } else {
                destination_cup = cup_to_next_cup.len() - 1;
            }
        }

        let previous_dest_next = cup_to_next_cup[destination_cup];
        cup_to_next_cup[destination_cup] = picked_up[0];
        cup_to_next_cup[picked_up[2]] = previous_dest_next;

        current_cup = cup_to_next_cup[current_cup];
    }
}

fn main() {
    let initial_cup_labels = [9, 7, 4, 6, 1, 8, 3, 5, 2];

    let mut cup_to_next_cup: Vec<usize> = vec![0; initial_cup_labels.len()];
    for (i, label) in initial_cup_labels.iter().map(|v| *v - 1).enumerate() {
        cup_to_next_cup[label] = initial_cup_labels[(i + 1) % initial_cup_labels.len()] - 1;
    }
    take_turns(&mut cup_to_next_cup, initial_cup_labels[0] - 1, 100);
    println!("Solution 1: {}", cups_to_string(&cup_to_next_cup));

    let mut cup_to_next_cup: Vec<usize> = vec![0; 1000000];
    for i in 0..cup_to_next_cup.len() {
        let label = if i >= initial_cup_labels.len() {
            i
        } else {
            initial_cup_labels[i] - 1
        };
        let next_label = if i + 1 >= cup_to_next_cup.len() {
            initial_cup_labels[i + 1 - cup_to_next_cup.len()] - 1
        } else if (i + 1) >= initial_cup_labels.len() {
            (i + 1) % cup_to_next_cup.len()
        } else {
            initial_cup_labels[i + 1] - 1
        };
        cup_to_next_cup[label] = next_label;
    }

    take_turns(&mut cup_to_next_cup, initial_cup_labels[0] - 1, 10000000);

    let star_1 = cup_to_next_cup[0];
    let star_2 = cup_to_next_cup[star_1];

    println!(
        "Solution 2: {}*{}={}",
        star_1 + 1,
        star_2 + 1,
        (star_1 + 1) * (star_2 + 1)
    );
}
