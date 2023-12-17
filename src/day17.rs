#![allow(dead_code)]

use std::{
    cmp::min,
    collections::{BTreeMap, HashMap},
    str::FromStr,
};

use adv2023::Pos;

#[derive(Debug)]
struct Map {
    blocks: Vec<Vec<u8>>,
    size: Pos,
    end: Pos,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blocks: Vec<Vec<u8>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();
        let size = Pos::new(blocks.len(), blocks[0].len());
        let end = size + Pos::new(-1, -1);
        Ok(Map { blocks, size, end })
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct State {
    pos: Pos,
    dir: Pos,
    moves_this_dir: u8,
}

impl State {
    fn go(&self, dir: Pos) -> Self {
        Self {
            pos: self.pos + dir,
            dir,
            moves_this_dir: if dir == self.dir {
                self.moves_this_dir + 1
            } else {
                1
            },
        }
    }
}

#[derive(Debug)]
struct Search<'a> {
    map: &'a Map,
    best_heat_loss: HashMap<State, usize>,
    to_visit: BTreeMap<State, usize>,
}

impl<'a> Search<'a> {
    fn new(map: &'a Map) -> Search<'a> {
        Self {
            map,
            best_heat_loss: HashMap::new(),
            to_visit: BTreeMap::new(),
        }
    }

    fn maybe_add_new_state(&mut self, state: State, heat_loss: usize) {
        if !state.pos.check_bounds(&self.map.size) {
            return;
        }
        let new_heat_loss =
            heat_loss + self.map.blocks[state.pos.y as usize][state.pos.x as usize] as usize;
        // Check if we already have visited the state with proposed or better heat loss.
        if let Some(hl) = self.best_heat_loss.get(&state) {
            if *hl <= new_heat_loss {
                return;
            }
        }
        // Check if we already have a pending visit, insert or update with best heat loss.
        let hl_ref = self.to_visit.entry(state).or_insert(usize::MAX);
        *hl_ref = min(*hl_ref, new_heat_loss);
    }

    fn search(&mut self, generate_moves: fn(&State) -> Vec<State>) -> usize {
        self.best_heat_loss.clear();
        self.to_visit.clear();
        self.to_visit.insert(
            State {
                pos: Pos::new(0, 0),
                dir: Pos::right(),
                moves_this_dir: 0,
            },
            0,
        );

        while let Some((state, heat_loss)) = self.to_visit.pop_first() {
            // println!("state: {:?} heat_loss: {}", &state, heat_loss);
            let v = self
                .best_heat_loss
                .entry(state.clone())
                .or_insert(usize::MAX);
            *v = heat_loss;

            for m in generate_moves(&state) {
                self.maybe_add_new_state(m, heat_loss);
            }
        }
        self.best_heat_loss
            .iter()
            .filter(|(k, _)| k.pos == self.map.end)
            .map(|(_, v)| *v)
            .min()
            .unwrap()
    }
}

fn part1_generate_moves(state: &State) -> Vec<State> {
    let mut out = Vec::new();
    if state.moves_this_dir < 3 {
        out.push(state.go(state.dir));
    }
    out.push(state.go(state.dir.cw()));
    out.push(state.go(state.dir.ccw()));
    out
}

fn part1(map: &Map) -> usize {
    let mut search = Search::new(map);
    search.search(part1_generate_moves)
}

fn part2_generate_moves(state: &State) -> Vec<State> {
    let mut out = Vec::new();
    if state.moves_this_dir < 10 {
        out.push(state.go(state.dir));
    }
    // moves_this_dir == 0 is special case for starting point where we can turn.
    if state.moves_this_dir == 0 || state.moves_this_dir >= 4 {
        out.push(state.go(state.dir.cw()));
        out.push(state.go(state.dir.ccw()));
    }
    out
}

fn part2(map: &Map) -> usize {
    let mut search = Search::new(map);
    search.search(part2_generate_moves)
}

fn main() {
    let input = adv2023::read_input();
    let map: Map = input.parse().unwrap();

    // dbg!(&map);
    // dbg!(part1(&map));
    dbg!(part2(&map));
}
