use itertools::Itertools;
use std::{ops::Range, str::FromStr};

type R64 = Range<u64>;

// One mapping from a source range to destination range.
#[derive(Debug)]
struct OneMapping {
    dest: R64,
    src: R64,
}

impl FromStr for OneMapping {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dest_start, src_start, len) = s
            .split(' ')
            .map(|x| x.parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap();
        Ok(Self {
            dest: dest_start..dest_start + len,
            src: src_start..src_start + len,
        })
    }
}

impl OneMapping {
    // Tries mapping given range, returns (translated ranges, remaining ranges).
    //
    // Possible outcomes, based on overlaps between "from" and self.src_range.
    // "T" denotes translated range, "R" denotes remaining range.
    //
    // 1. |-from-|   |-src-|
    //    RRRRRRRR
    //
    // 2. |-src-|   |-from-|
    //              RRRRRRRR
    //
    // 3. |-from-|
    //        |-src-|
    //    RRRRTTTT
    //
    // 4.     |-from-|
    //    |-src-|
    //        TTTRRRRR
    //
    // 5. |---from---|
    //      |-src-|
    //    RRTTTTTTTRRR
    //
    // 6.   |-from-|     |--from--|
    //    |---src----|   |--src---|
    //      TTTTTTTT     TTTTTTTTTT
    fn map(&self, from: &R64) -> (Vec<R64>, Vec<R64>) {
        let mut trans: Vec<R64> = vec![];
        let mut rem: Vec<R64> = vec![];
        // First, any range in "from" that's before "src", goes into remaining (cases 1, 3, 5).
        if from.start < self.src.start {
            let min_end = self.src.start.min(from.end);
            rem.push(from.start..min_end);
        }
        // Then, any range that overlaps goes into translated (cases 3, 4, 5, 6).
        if self.src.start < from.end && from.start < self.src.end {
            let max_start = self.src.start.max(from.start);
            let min_end = self.src.end.min(from.end);
            let trans_start = max_start - self.src.start + self.dest.start;
            let trans_end = min_end - self.src.start + self.dest.start;
            trans.push(trans_start..trans_end);
        }
        // Finally, any range in "from" that's after "src", goes into remaining (cases 2, 4, 5).
        if from.end > self.src.end {
            let max_start = self.src.end.max(from.start);
            rem.push(max_start..from.end);
        }
        (trans, rem)
    }
}

// One "X-to-Y map:" with multiple mappings.
#[derive(Debug)]
struct MappingSet {
    ranges: Vec<OneMapping>,
}

impl FromStr for MappingSet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut t = s.lines();
        // ignore first line that contains "x-to-y map:"
        t.next();
        let ranges = t.map(|line| line.parse().unwrap()).collect();
        Ok(Self { ranges })
    }
}

impl MappingSet {
    fn map(&self, from: &R64) -> Vec<R64> {
        // debug!("mapping {:?} using {:?}", from, self.ranges);
        let mut rem = vec![from.clone()];
        let mut trans: Vec<R64> = vec![];

        for range in &self.ranges {
            let mut next_rem: Vec<R64> = vec![];
            for r in &rem {
                let (this_trans, this_rem) = range.map(r);
                next_rem.extend(this_rem);
                trans.extend(this_trans);
            }
            rem = next_rem;
            // debug!(
            //     "after applying {:?}, rem={:?} trans={:?}",
            //     range, rem, trans
            // );
        }
        trans.extend(rem);
        trans
    }

    fn map_ranges(&self, from: &[R64]) -> Vec<R64> {
        from.iter().map(|r| self.map(r)).concat()
    }
}

#[derive(Debug)]
struct Problem {
    seeds: Vec<R64>,
    // We assume the maps are in order from seed->location.
    maps: Vec<MappingSet>,
}

// Parses "seeds: N N N..." line as individual seeds.
fn parse_seeds_part1(s: &str) -> Vec<R64> {
    s.trim_start_matches("seeds: ")
        .split(' ')
        .map(|x| {
            let y = x.parse().unwrap();
            y..(y + 1)
        })
        .collect()
}

// Parses "seeds: N M N M ..." line as (start, length) pairs.
fn parse_seeds_part2(s: &str) -> Vec<R64> {
    s.trim_start_matches("seeds: ")
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .chunks(2)
        .into_iter()
        // he was so preoccupied with whether he could and didn't stop to think if he should...
        .map(|chunk| {
            let (start, len) = chunk.collect_tuple().unwrap();
            start..start + len
        })
        .collect()
}

fn parse_problem(s: &str, seed_parser: fn(&str) -> Vec<R64>) -> Problem {
    let mut it = s.split("\n\n");
    let seeds = seed_parser(it.next().unwrap());
    let maps: Vec<MappingSet> = it.map(|x| x.parse().unwrap()).collect();
    Problem { seeds, maps }
}

impl Problem {
    fn lowest_location(&self) -> u64 {
        let ranges_for_each_seed = self
            .seeds
            .iter()
            .map(|seed| self.map_range_to_locations(seed));
        let min_location_for_each_seed =
            ranges_for_each_seed.map(|v| v.iter().map(|range| range.start).min().unwrap());
        min_location_for_each_seed.min().unwrap()
    }

    fn map_range_to_locations(&self, seeds: &R64) -> Vec<R64> {
        let mut values = vec![seeds.clone()];
        for map in &self.maps {
            values = map.map_ranges(&values);
        }
        values
    }
}

fn main() {
    let input = adv2023::read_input();
    let part1 = parse_problem(&input, parse_seeds_part1);
    // dbg!(&part1);
    dbg!(part1.lowest_location());

    let part2 = parse_problem(&input, parse_seeds_part2);
    // dbg!(&part2);
    dbg!(part2.lowest_location());
}
