use std::collections::HashMap;

use adv2023::read_input;

#[derive(Debug)]
struct Cubes {
    count: u32,
    color: String,
}

impl Cubes {
    // Parses "<n> <color>".
    fn parse(s: &str) -> Self {
        let (count_str, color) = s.trim().split_once(' ').unwrap();
        let count: u32 = count_str.parse().unwrap();
        Self {
            count,
            color: color.to_string(),
        }
    }
}

#[derive(Debug)]
struct Round {
    cubes: Vec<Cubes>,
}

type Limits = HashMap<String, u32>;

impl Round {
    fn parse(s: &str) -> Self {
        let cubes = s.split(',').map(|x| Cubes::parse(x)).collect();
        Self { cubes }
    }

    fn fits(&self, limits: &Limits) -> bool {
        self.cubes
            .iter()
            .all(|cubes| &cubes.count <= limits.get(&cubes.color).unwrap())
    }

    fn limits(&self) -> Limits {
        Limits::from_iter(
            self.cubes
                .iter()
                .map(|cubes| (cubes.color.clone(), cubes.count)),
        )
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn parse(line: &str) -> Self {
        let (game_and_id, rest) = line.split_once(":").unwrap();
        let (_, id_str) = game_and_id.split_once(" ").unwrap();
        let id: u32 = id_str.parse().unwrap();
        let rounds = rest.split(';').map(|x| Round::parse(x)).collect();
        Self { id, rounds }
    }

    fn fits(&self, limits: &Limits) -> bool {
        self.rounds.iter().all(|round| round.fits(limits))
    }

    fn limits(&self) -> Limits {
        self.rounds
            .iter()
            .map(|round| round.limits())
            .fold(Limits::new(), merge_limits)
    }

    fn power(&self) -> u32 {
        let limits = self.limits();
        limits.iter().fold(1, |acc, (_, v)| acc * v)
    }
}

fn merge_limits(mut limit1: Limits, limit2: Limits) -> Limits {
    limit2.into_iter().for_each(|(k, v)| {
        if limit1.contains_key(&k) {
            let v1 = limit1.get_mut(&k).unwrap();
            if v > *v1 {
                *v1 = v;
            }
        } else {
            limit1.insert(k, v);
        }
    });
    limit1
}

fn main() {
    let input = read_input();
    let games: Vec<Game> = input.lines().map(|line| Game::parse(line)).collect();
    let mut limits = Limits::new();
    limits.insert("red".to_string(), 12);
    limits.insert("green".to_string(), 13);
    limits.insert("blue".to_string(), 14);
    let part1: u32 = games
        .iter()
        .filter(|game| game.fits(&limits))
        .map(|game| game.id)
        .sum();
    dbg!(part1);
    let part2: u32 = games.iter().map(|game| game.power()).sum();
    dbg!(part2);
}
