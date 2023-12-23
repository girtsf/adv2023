use std::{collections::HashSet, str::FromStr};

use adv2023::Pos;

#[derive(Debug, Clone)]
struct Map {
    rocks: HashSet<Pos>,
    size: Pos,
    start: Pos,
}

type Reachable = HashSet<Pos>;

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        let size = Pos::new(grid.len(), grid[0].len());
        let mut rocks = HashSet::<Pos>::new();
        let mut start = None;
        for y in 0..size.y {
            for x in 0..size.x {
                let pos = Pos::new(y, x);
                match grid[y as usize][x as usize] {
                    'S' => {
                        start = Some(pos);
                    }
                    '#' => {
                        rocks.insert(pos);
                    }
                    '.' => {}
                    _ => panic!(),
                }
            }
        }
        let start = start.unwrap();
        Ok(Map { rocks, size, start })
    }
}

impl Map {
    fn steps(&self, from: &Reachable) -> Reachable {
        let mut next = Reachable::new();
        for pos in from.iter() {
            let poses_maybe = [
                pos + &Pos::up(),
                pos + &Pos::down(),
                pos + &Pos::left(),
                pos + &Pos::right(),
            ];
            for pos2 in poses_maybe {
                if pos2.check_bounds(&self.size) && !self.rocks.contains(&pos2) {
                    next.insert(pos2);
                }
            }
        }
        next
    }
}

fn main() {
    let input = adv2023::read_input();
    let map: Map = input.parse().unwrap();
    // dbg!(&map);
    let mut reachable = Reachable::from([map.start.clone()]);
    for _ in 0..64 {
        reachable = map.steps(&reachable);
        // dbg!(&reachable);
    }
    dbg!(&reachable.len());
}
