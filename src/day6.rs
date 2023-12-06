#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    fn count_ways_to_win(&self) -> u64 {
        let mut wins = 0u64;
        for t in 0..=self.time {
            let dist = t * (self.time - t);
            if dist > self.distance {
                wins += 1;
            }
        }
        wins
    }
}

fn calculate_ways_to_win_product(races: &[Race]) -> u64 {
    races.iter().map(|r| r.count_ways_to_win()).product::<u64>()
}

fn main() {
    let sample_part1 = vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)];
    dbg!(&calculate_ways_to_win_product(&sample_part1));

    let input_part1 = vec![
        Race::new(41, 214),
        Race::new(96, 1789),
        Race::new(88, 1127),
        Race::new(94, 1055),
    ];
    dbg!(&calculate_ways_to_win_product(&input_part1));

    let sample_part2 = vec![Race::new(71530, 940200)];
    dbg!(&calculate_ways_to_win_product(&sample_part2));

    let input_part2 = vec![Race::new(41968894, 214178911271055)];
    dbg!(&calculate_ways_to_win_product(&input_part2));
}
