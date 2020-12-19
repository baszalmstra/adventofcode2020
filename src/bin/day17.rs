use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Hash, Debug, Clone, Copy, Eq, PartialEq)]
struct Point3(isize, isize, isize);

#[derive(Hash, Debug, Clone, Copy, Eq, PartialEq)]
struct Point4(isize, isize, isize, isize);

fn max_point3(p: impl IntoIterator<Item = Point3>) -> Option<Point3> {
    let mut iter = p.into_iter();
    if let Some(v) = iter.next() {
        Some(iter.fold(v, |s, v| Point3(v.0.max(s.0), v.1.max(s.1), v.2.max(s.2))))
    } else {
        None
    }
}

fn min_point3(p: impl IntoIterator<Item = Point3>) -> Option<Point3> {
    let mut iter = p.into_iter();
    if let Some(v) = iter.next() {
        Some(iter.fold(v, |s, v| Point3(v.0.min(s.0), v.1.min(s.1), v.2.min(s.2))))
    } else {
        None
    }
}

fn max_point4(p: impl IntoIterator<Item = Point4>) -> Option<Point4> {
    let mut iter = p.into_iter();
    if let Some(v) = iter.next() {
        Some(iter.fold(v, |s, v| {
            Point4(v.0.max(s.0), v.1.max(s.1), v.2.max(s.2), v.3.max(s.3))
        }))
    } else {
        None
    }
}

fn min_point4(p: impl IntoIterator<Item = Point4>) -> Option<Point4> {
    let mut iter = p.into_iter();
    if let Some(v) = iter.next() {
        Some(iter.fold(v, |s, v| {
            Point4(v.0.min(s.0), v.1.min(s.1), v.2.min(s.2), v.3.min(s.3))
        }))
    } else {
        None
    }
}

impl Point3 {
    fn neighbours(&self) -> impl IntoIterator<Item = Point3> {
        let center = *self;
        let cx = self.0;
        let cy = self.1;
        let cz = self.2;
        (-1..=1)
            .flat_map(move |z| {
                (-1..=1).flat_map(move |y| (-1..=1).map(move |x| Point3(x + cx, y + cy, z + cz)))
            })
            .filter(move |p| *p != center)
    }
}

impl Point4 {
    fn neighbours(&self) -> impl IntoIterator<Item = Point4> {
        let center = *self;
        let cx = self.0;
        let cy = self.1;
        let cz = self.2;
        let cw = self.3;
        (-1..=1)
            .flat_map(move |w| {
                (-1..=1).flat_map(move |z| {
                    (-1..=1).flat_map(move |y| {
                        (-1..=1).map(move |x| Point4(x + cx, y + cy, z + cz, w + cw))
                    })
                })
            })
            .filter(move |p| *p != center)
    }
}

fn radiate3(active: &[Point3]) -> HashMap<Point3, usize> {
    let mut result = HashMap::default();
    for p in active {
        for n in p.neighbours() {
            result.entry(n).and_modify(|v| *v += 1).or_insert(1);
        }
    }
    result
}

fn radiate4(active: &[Point4]) -> HashMap<Point4, usize> {
    let mut result = HashMap::default();
    for p in active {
        for n in p.neighbours() {
            result.entry(n).and_modify(|v| *v += 1).or_insert(1);
        }
    }
    result
}

fn update3(active: &[Point3]) -> Vec<Point3> {
    let radiated = radiate3(active);

    let active: HashSet<Point3> = HashSet::from_iter(active.iter().copied());
    let min_pos = min_point3(radiated.keys().copied()).unwrap();
    let max_pos = max_point3(radiated.keys().copied()).unwrap();

    let mut result = Vec::new();

    for z in min_pos.2..=max_pos.2 {
        for y in min_pos.1..=max_pos.1 {
            for x in min_pos.0..=max_pos.0 {
                let p = Point3(x, y, z);
                let active_neighbours = radiated.get(&p).copied().unwrap_or(0);
                if active.contains(&p) {
                    if active_neighbours == 2 || active_neighbours == 3 {
                        result.push(p);
                    }
                } else if active_neighbours == 3 {
                    result.push(p);
                }
            }
        }
    }

    result
}

fn update4(active: &[Point4]) -> Vec<Point4> {
    let radiated = radiate4(active);

    let active: HashSet<Point4> = HashSet::from_iter(active.iter().copied());
    let min_pos = min_point4(radiated.keys().copied()).unwrap();
    let max_pos = max_point4(radiated.keys().copied()).unwrap();

    let mut result = Vec::new();

    for w in min_pos.3..=max_pos.3 {
        for z in min_pos.2..=max_pos.2 {
            for y in min_pos.1..=max_pos.1 {
                for x in min_pos.0..=max_pos.0 {
                    let p = Point4(x, y, z, w);
                    let active_neighbours = radiated.get(&p).copied().unwrap_or(0);
                    if active.contains(&p) {
                        if active_neighbours == 2 || active_neighbours == 3 {
                            result.push(p);
                        }
                    } else if active_neighbours == 3 {
                        result.push(p);
                    }
                }
            }
        }
    }

    result
}

fn main() {
    let input = std::fs::read_to_string("inputs/day17/input").unwrap();
    let initial_state: Vec<Point3> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Point3(x as isize, y as isize, 0))
                } else {
                    None
                }
            })
        })
        .collect();

    let mut state = initial_state.clone();
    for _cycle in 0..6 {
        state = update3(&state);
    }

    println!("Solution 1: {}", state.len());

    let mut state = initial_state
        .iter()
        .map(|p| Point4(p.0, p.1, p.2, 0))
        .collect::<Vec<_>>();
    for _cycle in 0..6 {
        state = update4(&state);
    }

    println!("Solution 2: {}", state.len());
}
