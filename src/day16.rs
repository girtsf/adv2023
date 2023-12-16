use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use adv2023::Pos;

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<char>>,
    size: Pos,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();
        let size = Pos::new(tiles.len(), tiles[0].len());
        Ok(Self { tiles, size })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Beam {
    pos: Pos,
    dir: Pos,
}

impl Map {
    fn count_energized(&self, start: &Beam) -> usize {
        let mut to_visit = VecDeque::<Beam>::from([start.clone()]);
        let mut seen = HashSet::<Beam>::from([start.clone()]);
        while let Some(beam) = to_visit.pop_front() {
            // println!("beam: {:?}", beam);
            let y = beam.pos.y as usize;
            let x = beam.pos.x as usize;
            let new_tiles = match self.tiles[y][x] {
                '.' => vec![Beam {
                    pos: beam.pos + beam.dir,
                    dir: beam.dir,
                }],
                '/' => {
                    // (0, 1) becomes (-1, 0)
                    // (0, -1) becomes (1, 0)
                    // (1, 0) becomes (0, -1)
                    // (-1, 0) becomes (0, 1)
                    let dir = Pos::new(-beam.dir.x, -beam.dir.y);
                    vec![Beam {
                        pos: beam.pos + dir,
                        dir,
                    }]
                }
                '\\' => {
                    // (0, 1) becomes (1, 0)
                    // (0, -1) becomes (-1, 0)
                    // (1, 0) becomes (0, 1)
                    // (-1, 0) becomes (0, -1)
                    let dir = Pos::new(beam.dir.x, beam.dir.y);
                    vec![Beam {
                        pos: beam.pos + dir,
                        dir,
                    }]
                }
                '|' => {
                    if beam.dir.y != 0 {
                        // Passing through.
                        vec![Beam {
                            pos: beam.pos + beam.dir,
                            dir: beam.dir,
                        }]
                    } else {
                        // Splitting.
                        vec![
                            Beam {
                                pos: beam.pos + Pos::up(),
                                dir: Pos::up(),
                            },
                            Beam {
                                pos: beam.pos + Pos::down(),
                                dir: Pos::down(),
                            },
                        ]
                    }
                }
                '-' => {
                    if beam.dir.x != 0 {
                        // Passing through.
                        vec![Beam {
                            pos: beam.pos + beam.dir,
                            dir: beam.dir,
                        }]
                    } else {
                        // Splitting.
                        vec![
                            Beam {
                                pos: beam.pos + Pos::left(),
                                dir: Pos::left(),
                            },
                            Beam {
                                pos: beam.pos + Pos::right(),
                                dir: Pos::right(),
                            },
                        ]
                    }
                }
                x => {
                    panic!("wtf? {x}");
                }
            };
            // println!("new tiles: {:?}", &new_tiles);
            for tile in new_tiles {
                if seen.contains(&tile) {
                    continue;
                }
                if !tile.pos.check_bounds(&self.size) {
                    continue;
                }
                seen.insert(tile.clone());
                to_visit.push_back(tile);
            }
        }
        let seen_tiles: HashSet<Pos> = seen.iter().map(|beam| beam.pos).collect();
        seen_tiles.len()
    }
}

fn part1(map: &Map) -> usize {
    let start = Beam {
        pos: Pos::new(0, 0),
        dir: Pos::new(0, 1),
    };
    map.count_energized(&start)
}

fn part2(map: &Map) -> usize {
    let mut max = 0usize;
    for y in 0..map.size.y {
        let start = Beam {
            pos: Pos::new(y, 0),
            dir: Pos::new(0, 1),
        };
        max = max.max(map.count_energized(&start));
        let start = Beam {
            pos: Pos::new(y, map.size.x - 1),
            dir: Pos::new(0, -1),
        };
        max = max.max(map.count_energized(&start));
    }
    for x in 0..map.size.x {
        let start = Beam {
            pos: Pos::new(0, x),
            dir: Pos::new(1, 0),
        };
        max = max.max(map.count_energized(&start));
        let start = Beam {
            pos: Pos::new(map.size.y - 1, x),
            dir: Pos::new(-1, 0),
        };
        max = max.max(map.count_energized(&start));
    }
    max
}

fn main() {
    let input = adv2023::read_input();
    let map: Map = input.parse().unwrap();
    dbg!(part1(&map));
    dbg!(part2(&map));
}
