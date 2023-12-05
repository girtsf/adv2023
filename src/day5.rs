use std::str::FromStr;

#[derive(Debug)]
struct Range {
    dest_range_start: u64,
    src_range_start: u64,
    len: u64,
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ss: Vec<_> = s.split(' ').collect();
        assert_eq!(ss.len(), 3);
        Ok(Self {
            dest_range_start: ss[0].parse().unwrap(),
            src_range_start: ss[1].parse().unwrap(),
            len: ss[2].parse().unwrap(),
        })
    }
}

impl Range {
    fn try_map(&self, src: u64) -> Option<u64> {
        if src >= self.src_range_start && src < (self.src_range_start + self.len) {
            Some(self.dest_range_start + (src - self.src_range_start))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut t = s.lines();
        // ignore first line that contains "x-to-y map:"
        t.next();
        let ranges = t.map(|line| line.parse().unwrap()).collect();
        Ok(Self { ranges })
    }
}

impl Map {
    fn map(&self, src: u64) -> u64 {
        for range in &self.ranges {
            if let Some(dest) = range.try_map(src) {
                return dest;
            }
        }
        src
    }
}

#[derive(Debug)]
struct Problem {
    seeds: Vec<u64>,
    // We assume the maps are in order from seed->location.
    maps: Vec<Map>,
}

fn parse_seeds(s: &str) -> Vec<u64> {
    s.trim_start_matches("seeds: ")
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect()
}

impl FromStr for Problem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split("\n\n");
        let seeds = parse_seeds(it.next().unwrap());
        let maps: Vec<Map> = it.map(|x| x.parse().unwrap()).collect();
        Ok(Self { seeds, maps })
    }
}

impl Problem {
    fn part1(&self) -> u64 {
        self.seeds
            .iter()
            .map(|seed| self.map_seed_to_location(*seed))
            .min()
            .unwrap()
    }

    fn map_seed_to_location(&self, seed: u64) -> u64 {
        dbg!(&seed);
        let mut value = seed;
        for map in &self.maps {
            value = map.map(value);
            dbg!(&value);
        }
        value
    }
}

fn main() {
    let input = adv2023::read_input();
    let problem: Problem = input.parse().unwrap();
    dbg!(&problem);
    dbg!(problem.part1());
}
