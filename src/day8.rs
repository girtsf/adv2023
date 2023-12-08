use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;
use num::integer::lcm;

#[derive(Debug)]
struct Network {
    instructions: String,
    network: HashMap<String, (String, String)>,
}

impl FromStr for Network {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (i, n) = s.split_once("\n\n").unwrap();
        let instructions = i.to_string();
        let network = n
            .lines()
            .map(|line| {
                let (from, to) = line.trim_matches(')').split_once(" = (").unwrap();
                let tos = to
                    .split(", ")
                    .map(|x| x.to_string())
                    .collect_tuple()
                    .unwrap();
                (from.to_string(), tos)
            })
            .collect();
        Ok(Self {
            instructions,
            network,
        })
    }
}

impl Network {
    fn find_path(&self, from: &str, to_pred: fn(&str) -> bool) -> usize {
        let mut loc = from;
        let mut steps = 0usize;
        while !to_pred(loc) {
            let idx = steps % self.instructions.len();
            steps += 1;
            // dbg!(loc);
            let choices = self.network.get(loc).unwrap();
            loc = match self.instructions.chars().nth(idx).unwrap() {
                'L' => &choices.0,
                'R' => &choices.1,
                _ => panic!(),
            };
        }
        steps
    }

    fn part1(&self) -> usize {
        self.find_path("AAA", |path| path == "ZZZ")
    }

    fn part2(&self) -> usize {
        let starts: Vec<_> = self
            .network
            .keys()
            .filter(|k| k.ends_with('A'))
            .cloned()
            .collect();
        starts
            .iter()
            .map(|s| dbg!(self.find_path(&s, |path| path.ends_with('Z'))))
            .reduce(|acc, e| lcm(acc, e))
            .unwrap()
    }
}

fn main() {
    let input = adv2023::read_input();
    let network: Network = input.parse().unwrap();
    // dbg!(&network);
    dbg!(network.part1());
    dbg!(network.part2());
}
