use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    hash::Hasher,
    str::FromStr,
};

use adv2023::Pos;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum RockType {
    Round,
    Cube,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Rock {
    pos: Pos,
    rock_type: RockType,
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
struct Rocks(Vec<Rock>);

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Map {
    size: Pos,
    rocks: Rocks,
    // From position to index in 'rocks'.
    lookup: HashMap<Pos, usize>,
}

impl Hash for Map {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.size.hash(state);
        self.rocks.0.hash(state);
    }
}

impl Map {
    fn dump(&self) {
        println!("=============");
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let c = match self
                    .lookup
                    .get(&Pos::new(y, x))
                    .map(|idx| &self.rocks.0[*idx].rock_type)
                {
                    Some(&RockType::Cube) => '#',
                    Some(&RockType::Round) => 'O',
                    None => ' ',
                };
                print!("{}", c);
            }
            println!();
        }
        println!("=============");
        println!();
    }
    fn add_rock(&mut self, rock: Rock) {
        assert!(self
            .lookup
            .insert(rock.pos.clone(), self.rocks.0.len())
            .is_none());
        self.rocks.0.push(rock);
    }

    fn move_rock(&mut self, from: Pos, to: Pos) {
        let idx = self.lookup.remove(&from).unwrap();
        self.rocks.0[idx].pos = to.clone();
        assert!(self.lookup.insert(to, idx).is_none());
    }

    // fn roll(&mut self, dir: &Pos) {
    //     let mut moved = true;
    //     while moved {
    //         moved = false;
    //         for i in 0..self.rocks.len() {
    //             let rock = &self.rocks[i];
    //             if rock.rock_type == RockType::Cube {
    //                 continue;
    //             }
    //             let to = rock.pos + *dir;
    //             if to.y < 0 || to.y >= self.size.y || to.x < 0 || to.x >= self.size.x {
    //                 continue;
    //             }
    //             if !self.lookup.contains_key(&to) {
    //                 self.move_rock(rock.pos, to);
    //                 moved = true;
    //             }
    //         }
    //     }
    // }

    fn roll(&mut self, dir: &Pos) {
        let mut moved = true;
        while moved {
            moved = false;
            for i in 0..self.rocks.0.len() {
                let rock = &self.rocks.0[i];
                let mut pos = rock.pos;
                if rock.rock_type == RockType::Cube {
                    continue;
                }
                loop {
                    let to = pos + *dir;
                    if to.y < 0 || to.y >= self.size.y || to.x < 0 || to.x >= self.size.x {
                        break;
                    }
                    if self.lookup.contains_key(&to) {
                        break;
                    }
                    self.move_rock(pos, to);
                    moved = true;
                    pos = to;
                }
            }
        }
    }

    // fn find_next_space(&self, pos: &Pos, dir: &Pos) -> Option<Pos> {
    //     let mut pos = pos.clone();
    //     while pos.check_bounds(&self.size) {
    //         if !self.lookup.contains_key(&pos) {
    //             return Some(pos);
    //         }
    //         pos += dir;
    //     }
    //     None
    // }

    // fn find_next_round_rock(&self, pos: &Pos, dir: &Pos) -> Option<Pos> {
    //     let mut pos = pos.clone();
    //     while pos.check_bounds(&self.size) {
    //         if !self.lookup.contains_key(&pos) {
    //             return Some(pos);
    //         }
    //         pos += dir;
    //     }
    //     None
    // }

    fn calculate_load(&self) -> isize {
        self.rocks
            .0
            .iter()
            .filter(|r| r.rock_type == RockType::Round)
            .map(|r| self.size.y as isize - r.pos.y as isize)
            .sum()
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let size = Pos::new(lines.len(), lines[0].len());
        let mut map = Map {
            size,
            ..Default::default()
        };
        lines.iter().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let pos = Pos::new(y, x);
                let rock_type = if c == 'O' {
                    RockType::Round
                } else if c == '#' {
                    RockType::Cube
                } else {
                    return;
                };
                map.add_rock(Rock { pos, rock_type });
            });
        });
        Ok(map)
    }
}

fn part1(map: &Map) -> isize {
    let mut map = map.clone();
    map.roll(&Pos::up());
    map.calculate_load()
}

fn cycle(map: &mut Map) {
    map.roll(&Pos::up());
    map.roll(&Pos::left());
    map.roll(&Pos::down());
    map.roll(&Pos::right());
}

fn part2(map: &Map) -> isize {
    let mut map = map.clone();
    let mut states_seen = HashMap::<Rocks, usize>::new();

    for i in 0..1000000 {
        dbg!(i);
        let mut rocks = map.rocks.clone();
        rocks.0.sort();
        if let Some(prev_i) = states_seen.get(&rocks) {
            let delta = i - prev_i;
            let rem = (1_000_000_000 - i) % delta;
            for _ in 0..rem {
                cycle(&mut map);
            }
            return map.calculate_load();
        }
        states_seen.insert(rocks, i);
        cycle(&mut map);
    }
    todo!()
}

fn main() {
    let input = adv2023::read_input();
    let map: Map = input.parse().unwrap();
    map.dump();

    dbg!(part1(&map));
    dbg!(part2(&map));
}
