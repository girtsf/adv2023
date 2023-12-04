use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    winning: HashSet<u32>,
    have: Vec<u32>,
    // for part 2
    count: u32,
}

fn parse_ints(s: &str) -> Vec<u32> {
    s.trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

impl Card {
    fn parse(s: &str) -> Self {
        let after_colon = s.split_once(':').unwrap().1;
        let (w, h) = after_colon.split_once(" | ").unwrap();
        Self {
            winning: HashSet::from_iter(parse_ints(w).into_iter()),
            have: parse_ints(h),
            count: 1,
        }
    }

    fn count_matches(&self) -> u32 {
        self.have
            .iter()
            .filter(|h| self.winning.contains(h))
            .count() as u32
    }

    fn score_part1(&self) -> u32 {
        match self.count_matches() {
            0 => 0,
            count => 2u32.pow(count - 1),
        }
    }
}

fn part1(cards: &[Card]) -> u32 {
    cards.iter().map(|c| c.score_part1()).sum()
}

fn part2(cards: &mut [Card]) -> u32 {
    for i in 0..cards.len() {
        let matches = cards[i].count_matches();
        for j in 0..matches {
            cards[i + 1 + j as usize].count += cards[i].count;
        }
    }
    cards.iter().map(|c| c.count).sum()
}

fn main() {
    let input = adv2023::read_input();
    let mut cards: Vec<Card> = input.lines().map(|l| Card::parse(l)).collect();
    dbg!(part1(&cards));
    dbg!(part2(&mut cards));
}
