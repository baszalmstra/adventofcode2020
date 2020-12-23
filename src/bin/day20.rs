use std::collections::{HashMap, HashSet};

type Image = [[bool; 10]; 10];

#[derive(Debug, Default)]
struct Edge {
    cw: u16,
    ccw: u16,
}

#[derive(Debug, Default)]
struct Tile {
    id: usize,
    image: Image,
    edges: [Edge; 4],
}

fn parse(input: &str) -> Vec<Tile> {
    let mut lines = input.lines();

    let mut result = Vec::new();
    while let Some(line) = lines.next() {
        let id: usize = line
            .strip_prefix("Tile ")
            .and_then(|str| str.strip_suffix(":"))
            .and_then(|id| id.parse().ok())
            .unwrap();

        let mut image = [[false; 10]; 10];
        for row in image.iter_mut() {
            let mut y_line = lines.next().unwrap().chars();
            for value in row.iter_mut() {
                *value = y_line.next().unwrap() == '#';
            }
        }

        let edges = [
            // Top edge
            (0..10).fold(Edge { cw: 0, ccw: 0 }, |Edge { cw, ccw }, i| {
                let value = if image[0][i] { 1 } else { 0 };
                Edge {
                    cw: cw | value << i,
                    ccw: ccw | value << (9 - i),
                }
            }),
            // Right edge
            (0..10).fold(Edge { cw: 0, ccw: 0 }, |Edge { cw, ccw }, i| {
                let value = if image[i][9] { 1 } else { 0 };
                Edge {
                    cw: cw | value << i,
                    ccw: ccw | value << (9 - i),
                }
            }),
            // Bottom edge
            (0..10).fold(Edge { cw: 0, ccw: 0 }, |Edge { cw, ccw }, i| {
                let value = if image[9][9 - i] { 1 } else { 0 };
                Edge {
                    cw: cw | value << i,
                    ccw: ccw | value << (9 - i),
                }
            }),
            // Left edge
            (0..10).fold(Edge { cw: 0, ccw: 0 }, |Edge { cw, ccw }, i| {
                let value = if image[9 - i][0] { 1 } else { 0 };
                Edge {
                    cw: cw | value << i,
                    ccw: ccw | value << (9 - i),
                }
            }),
        ];

        result.push(Tile { id, image, edges });

        lines.next();
    }

    result
}

static ALL_ORIENTATIONS: [TileOrientation; 8] = [
    TileOrientation {
        rotation: 0,
        flipped: true,
    },
    TileOrientation {
        rotation: 3,
        flipped: true,
    },
    TileOrientation {
        rotation: 0,
        flipped: false,
    },
    TileOrientation {
        rotation: 1,
        flipped: false,
    },
    TileOrientation {
        rotation: 2,
        flipped: false,
    },
    TileOrientation {
        rotation: 3,
        flipped: false,
    },
    TileOrientation {
        rotation: 1,
        flipped: true,
    },
    TileOrientation {
        rotation: 2,
        flipped: true,
    },
];

#[derive(Clone, Default, Copy)]
struct TileOrientation {
    rotation: usize,
    flipped: bool,
}

impl TileOrientation {
    fn get_top_edge(&self, tile: &Tile, counter: bool) -> u16 {
        let idx = match self.flipped {
            false => self.rotation,
            true => (2 + 4 - self.rotation) % 4,
        };
        if self.flipped ^ counter {
            tile.edges[idx].ccw
        } else {
            tile.edges[idx].cw
        }
    }

    fn get_right_edge(&self, tile: &Tile, counter: bool) -> u16 {
        let idx = match self.flipped {
            false => (1 + self.rotation) % 4,
            true => (1 + 4 - self.rotation) % 4,
        };
        if self.flipped ^ counter {
            tile.edges[idx].ccw
        } else {
            tile.edges[idx].cw
        }
    }

    fn get_bottom_edge(&self, tile: &Tile, counter: bool) -> u16 {
        let idx = match self.flipped {
            false => (2 + self.rotation) % 4,
            true => (4 - self.rotation) % 4,
        };
        if self.flipped ^ counter {
            tile.edges[idx].ccw
        } else {
            tile.edges[idx].cw
        }
    }

    fn get_left_edge(&self, tile: &Tile, counter: bool) -> u16 {
        let idx = match self.flipped {
            false => (3 + self.rotation) % 4,
            true => 3 - self.rotation,
        };
        if self.flipped ^ counter {
            tile.edges[idx].ccw
        } else {
            tile.edges[idx].cw
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("inputs/day20/input").unwrap();
    let tiles = parse(&input);

    let mut tiles_for_edge: HashMap<u16, Vec<usize>> = HashMap::new();
    for tile in tiles.iter() {
        for edge in tile.edges.iter() {
            tiles_for_edge
                .entry(edge.cw)
                .and_modify(|tiles| tiles.push(tile.id))
                .or_insert_with(|| vec![tile.id]);
            tiles_for_edge
                .entry(edge.ccw)
                .and_modify(|tiles| tiles.push(tile.id))
                .or_insert_with(|| vec![tile.id]);
        }
    }

    let puzzle_edge_edges = tiles_for_edge
        .iter()
        .filter(|(_, tiles)| tiles.len() == 1)
        .map(|(e, _)| *e)
        .collect::<HashSet<_>>();

    let corner_tiles = tiles
        .iter()
        .filter(|f| {
            f.edges
                .iter()
                .flat_map(|e| vec![e.cw, e.ccw].into_iter())
                .filter(|e| puzzle_edge_edges.contains(e))
                .count()
                == 4
        })
        .map(|t| t.id)
        .collect::<Vec<_>>();

    assert_eq!(corner_tiles.len(), 4);
    println!("Solution 1: {}", corner_tiles.iter().product::<usize>());

    let first_corner = tiles.iter().find(|t| corner_tiles.contains(&t.id)).unwrap();

    let width = (tiles.len() as f64).sqrt() as usize;

    let mut states = ALL_ORIENTATIONS
        .iter()
        .map(|orientation| vec![(first_corner, *orientation)])
        .collect::<Vec<_>>();
    for y in 0..width {
        for x in 0..width {
            if x == 0 && y == 0 {
                continue;
            }

            let mut new_states = Vec::new();
            for state in states {
                let top = if y > 0 {
                    Some(&state[(y - 1) * width + x])
                } else {
                    None
                };
                let left = if x > 0 {
                    Some(&state[y * width + x - 1])
                } else {
                    None
                };

                let top = top.map(|(tile, orientation)| orientation.get_bottom_edge(*tile, true));
                let left = left.map(|(tile, orientation)| orientation.get_right_edge(*tile, true));

                for tile in tiles.iter() {
                    if state
                        .iter()
                        .any(|(placed_tile, _)| placed_tile.id == tile.id)
                    {
                        continue;
                    }

                    for orientation in ALL_ORIENTATIONS.iter().copied() {
                        if let Some(top) = top {
                            if orientation.get_top_edge(&tile, false) != top {
                                continue;
                            }
                        }
                        if let Some(left) = left {
                            if orientation.get_left_edge(&tile, false) != left {
                                continue;
                            }
                        }

                        let mut new_state = state.clone();
                        new_state.push((&tile, orientation));
                        new_states.push(new_state);
                    }
                }
            }

            states = new_states;
        }
    }

    let tile_states = states.into_iter().next().unwrap();

    let final_width = width * 8;
    let mut final_image = vec![false; final_width * final_width];
    for y in 0..width {
        for x in 0..width {
            let (tile, orient) = tile_states[y * width + x];
            let mut tile_image = tile.image;
            if orient.flipped {
                tile_image = flip_tile_image(tile_image)
            }
            tile_image = rotate_tile_image(tile_image, orient.rotation);

            let x_offset = x * 8;
            let y_offset = y * 8;
            for iy in 0..8 {
                for ix in 0..8 {
                    final_image[(y_offset + iy) * final_width + (x_offset + ix)] =
                        tile_image[iy + 1][ix + 1]
                }
            }
        }
    }

    let count = find_sea_monsters(&final_image, final_width).unwrap();
    println!("Solution 2: {}", count);
}

fn find_sea_monsters(image: &[bool], width: usize) -> Option<usize> {
    let sea_monster_pattern = "                  # \n#    ##    ##    ###\n #  #  #  #  #  #   "
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| if c == '#' { Some((x, y)) } else { None })
        })
        .collect::<Vec<_>>();

    let max_y = sea_monster_pattern.iter().map(|(_, y)| *y).max().unwrap();
    let max_x = sea_monster_pattern.iter().map(|(x, _)| *x).max().unwrap();

    for orient in ALL_ORIENTATIONS.iter() {
        let mut image = rotate_image(image, width, orient.rotation);
        if orient.flipped {
            image = flip_image(&image, width);
        }

        let mut seamonsters = 0;
        for y in 0..width - max_y {
            for x in 0..width - max_x {
                if sea_monster_pattern
                    .iter()
                    .all(|(dx, dy)| image[(y + dy) * width + x + dx])
                {
                    for (dx, dy) in sea_monster_pattern.iter() {
                        image[(y + dy) * width + x + dx] = false;
                    }
                    seamonsters += 1;
                }
            }
        }

        if seamonsters > 0 {
            return Some(image.into_iter().filter(|b| *b).count());
        }
    }

    None
}

fn flip_image(image: &[bool], width: usize) -> Vec<bool> {
    let mut new_image = vec![false; image.len()];
    for y in 0..width {
        for x in 0..width {
            new_image[y * width + x] = image[((width - 1) - y) * width + x];
        }
    }
    new_image
}

fn rotate_image(image: &[bool], width: usize, rotations: usize) -> Vec<bool> {
    let mut result = image.to_vec();
    for _rot in 0..rotations {
        let mut new_image = vec![false; image.len()];
        for y in 0..width {
            for x in 0..width {
                new_image[y * width + x] = result[x * width + (width - 1) - y];
            }
        }
        result = new_image;
    }
    result
}

fn flip_tile_image(image: Image) -> Image {
    let mut result = [[false; 10]; 10];
    for (y, row) in result.iter_mut().enumerate() {
        *row = image[9 - y];
    }
    result
}

fn rotate_tile_image(image: Image, rotations: usize) -> Image {
    let mut new_image = image;
    for _rot in 0..rotations {
        let mut result = [[false; 10]; 10];
        for (y, row) in result.iter_mut().enumerate() {
            for (x, value) in row.iter_mut().enumerate() {
                *value = new_image[x][9 - y];
            }
        }
        new_image = result;
    }
    new_image
}
