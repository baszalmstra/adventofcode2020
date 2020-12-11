#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Floor,
    Empty,
    Taken,
    OutOfBounds,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Tiles {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Tiles {
    fn get_neighbours(&self, x: usize, y: usize) -> impl Iterator<Item = Tile> + '_ {
        (-1..=1)
            .flat_map(move |dx| (-1..=1).map(move |dy| (x as isize + dx, y as isize + dy)))
            .filter(move |(nx, ny)| *nx != x as isize || *ny != y as isize)
            .map(move |(x, y)| {
                if x >= self.width as isize || x < 0 || y >= self.height as isize || y < 0 {
                    Tile::OutOfBounds
                } else {
                    self.tiles[y as usize * self.width + x as usize]
                }
            })
    }

    fn get_first_seat_in_direction(
        &self,
        x: usize,
        y: usize,
        dx: isize,
        dy: isize,
    ) -> Option<(usize, usize)> {
        let mut x = x as isize + dx;
        let mut y = y as isize + dy;
        while x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize {
            match self.tiles[y as usize * self.width + x as usize] {
                Tile::Taken | Tile::Empty => return Some((x as usize, y as usize)),
                _ => {}
            }
            x += dx;
            y += dy;
        }
        None
    }

    fn get_line_of_sight_map(&self) -> Vec<Vec<usize>> {
        (0..self.height)
            .into_iter()
            .flat_map(|y| (0..self.width).into_iter().map(move |x| (x, y)))
            .map(|(x, y)| {
                (-1..=1)
                    .into_iter()
                    .flat_map(|dx| (-1..=1).into_iter().map(move |dy| (dx, dy)))
                    .filter(move |(dx, dy)| *dx != 0 || *dy != 0)
                    .filter_map(|(dx, dy)| self.get_first_seat_in_direction(x, y, dx, dy))
                    .map(|(x, y)| y * self.width + x)
                    .collect()
            })
            .collect()
    }
}

fn parse(input: &str) -> Tiles {
    let tiles: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Floor,
                    'L' => Tile::Empty,
                    '#' => Tile::Taken,
                    t => unreachable!("{}", t),
                })
                .collect()
        })
        .collect();

    let width = tiles[0].len();
    let height = tiles.len();

    Tiles {
        tiles: tiles.into_iter().flat_map(|v| v.into_iter()).collect(),
        width,
        height,
    }
}

fn tick(tiles: &Tiles) -> Tiles {
    let mut new_tiles = Tiles {
        tiles: vec![Tile::Floor; tiles.tiles.len()],
        width: tiles.width,
        height: tiles.height,
    };

    for y in 0..new_tiles.height {
        for x in 0..new_tiles.width {
            let idx = y * tiles.width + x;
            new_tiles.tiles[idx] = match tiles.tiles[idx] {
                Tile::Empty => {
                    if tiles.get_neighbours(x, y).any(|t| t == Tile::Taken) {
                        Tile::Empty
                    } else {
                        Tile::Taken
                    }
                }
                Tile::Taken => {
                    if tiles
                        .get_neighbours(x, y)
                        .filter(|t| *t == Tile::Taken)
                        .count()
                        >= 4
                    {
                        Tile::Empty
                    } else {
                        Tile::Taken
                    }
                }
                t => t,
            }
        }
    }

    new_tiles
}

fn tick_with_line_of_sight(tiles: &Tiles, line_of_sight: &[Vec<usize>]) -> Tiles {
    let mut new_tiles = Tiles {
        tiles: vec![Tile::Floor; tiles.tiles.len()],
        width: tiles.width,
        height: tiles.height,
    };

    for y in 0..new_tiles.height {
        for x in 0..new_tiles.width {
            let idx = y * tiles.width + x;
            new_tiles.tiles[idx] = match tiles.tiles[idx] {
                Tile::Empty => {
                    if line_of_sight[idx]
                        .iter()
                        .map(|idx| tiles.tiles[*idx])
                        .any(|t| t == Tile::Taken)
                    {
                        Tile::Empty
                    } else {
                        Tile::Taken
                    }
                }
                Tile::Taken => {
                    if line_of_sight[idx]
                        .iter()
                        .map(|idx| tiles.tiles[*idx])
                        .filter(|t| *t == Tile::Taken)
                        .count()
                        >= 5
                    {
                        Tile::Empty
                    } else {
                        Tile::Taken
                    }
                }
                t => t,
            }
        }
    }

    new_tiles
}

fn main() {
    let input = std::fs::read_to_string("inputs/day11/input").unwrap();
    let tiles = parse(&input);

    let count = {
        let mut tiles = tiles.clone();
        loop {
            let new_tiles = tick(&tiles);
            if new_tiles == tiles {
                break new_tiles
                    .tiles
                    .into_iter()
                    .filter(|t| *t == Tile::Taken)
                    .count();
            }
            tiles = new_tiles;
        }
    };

    println!("Solution 1: {}", count);

    let line_of_sight_map = tiles.get_line_of_sight_map();
    let count = {
        let mut tiles = tiles;
        loop {
            let new_tiles = tick_with_line_of_sight(&tiles, &line_of_sight_map);
            if new_tiles == tiles {
                break new_tiles
                    .tiles
                    .into_iter()
                    .filter(|t| *t == Tile::Taken)
                    .count();
            }
            tiles = new_tiles;
        }
    };

    println!("Solution 2: {}", count);
}
