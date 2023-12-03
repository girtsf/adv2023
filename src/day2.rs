use std::collections::HashMap;

use adv2023::read_input;

// Parses "<n> <color>".
fn parse_cube_count(s: &str) -> (String, u32) {
    let (count_str, color) = s.trim().split_once(' ').unwrap();
    let count: u32 = count_str.parse().unwrap();
    (color.to_string(), count)
}

#[derive(Debug, Clone)]
struct Cubes(HashMap<String, u32>);

impl Cubes {
    // Parses "," delimited "<n> <color>".
    fn parse(s: &str) -> Self {
        Cubes(HashMap::from_iter(
            s.split(',').map(|x| parse_cube_count(x)),
        ))
    }

    fn fits(&self, limits: &Cubes) -> bool {
        self.0
            .iter()
            .all(|(color, count)| count <= limits.0.get(color).unwrap())
    }

    fn merge_max(mut self, other: Cubes) -> Cubes {
        other.0.into_iter().for_each(|(color, count)| {
            self.0
                .entry(color)
                .and_modify(|v| *v = count.max(*v))
                .or_insert(count);
        });
        self
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Cubes>,
}

impl Game {
    fn parse(line: &str) -> Self {
        let (game_and_id, rest) = line.split_once(":").unwrap();
        let (_, id_str) = game_and_id.split_once(" ").unwrap();
        let id: u32 = id_str.parse().unwrap();
        let rounds = rest.split(';').map(|x| Cubes::parse(x)).collect();
        Self { id, rounds }
    }

    fn fits(&self, limits: &Cubes) -> bool {
        self.rounds.iter().all(|round| round.fits(limits))
    }

    fn min_cubes_needed(&self) -> Cubes {
        self.rounds
            .iter()
            .cloned()
            .reduce(Cubes::merge_max)
            .unwrap()
    }

    fn power(&self) -> u32 {
        self.min_cubes_needed()
            .0
            .values()
            .cloned()
            .reduce(|acc, v| acc * v)
            .unwrap()
    }
}

fn main() {
    let input = read_input();
    let games: Vec<Game> = input.lines().map(|line| Game::parse(line)).collect();
    let limits = Cubes::parse("12 red, 13 green, 14 blue");
    let part1: u32 = games
        .iter()
        .filter(|game| game.fits(&limits))
        .map(|game| game.id)
        .sum();
    dbg!(part1);
    let part2: u32 = games.iter().map(|game| game.power()).sum();
    dbg!(part2);
}
