use std::{collections::HashMap, str::FromStr};

use itertools::{interleave, repeat_n};

#[derive(Debug, Default)]
struct Record {
    cond: Vec<char>,
    groups: Vec<usize>,
}

impl FromStr for Record {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cond_s, groups_s) = s.split_once(' ').unwrap();
        let cond = cond_s.chars().collect();
        let groups = groups_s.split(',').map(|x| x.parse().unwrap()).collect();
        Ok(Self { cond, groups })
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Default)]
struct Path {
    // Index into self.groups.
    group_idx: usize,
    // Count within current group.
    group_cnt: usize,
}

impl Path {
    fn new(group_idx: usize, group_cnt: usize) -> Self {
        Self {
            group_idx,
            group_cnt,
        }
    }
}

type Paths = HashMap<Path, usize>;

impl Record {
    fn count_ways(&self) -> usize {
        let mut paths = Paths::from([(Path::default(), 1)]);
        for (_i, &c) in self.cond.iter().enumerate() {
            let mut new_paths = Paths::new();
            for (path, path_count) in &paths {
                if c == '.' || c == '?' {
                    // Try ".".
                    if path.group_cnt == 0 {
                        // Currently not in a group.
                        *new_paths.entry(path.clone()).or_default() += path_count;
                    } else if path.group_cnt == self.groups[path.group_idx] {
                        // Good to end a group.
                        *new_paths
                            .entry(Path::new(path.group_idx + 1, 0))
                            .or_default() += path_count;
                    }
                }
                if c == '#' || c == '?' {
                    // Try '#'.
                    if path.group_idx < self.groups.len()
                        && path.group_cnt < self.groups[path.group_idx]
                    {
                        *new_paths
                            .entry(Path::new(path.group_idx, path.group_cnt + 1))
                            .or_default() += path_count;
                    }
                }
            }
            // debug!("after {i}: {:?}", &new_paths);
            paths = new_paths;
        }
        paths
            .iter()
            .map(|(path, &count)| {
                if path.group_idx >= self.groups.len()
                    || ((path.group_idx == (self.groups.len() - 1))
                        && (path.group_cnt >= self.groups[path.group_idx]))
                {
                    count
                } else {
                    0
                }
            })
            .sum()
    }

    fn make_into_part2(&self) -> Record {
        let cond = interleave(repeat_n(self.cond.clone(), 5), repeat_n(vec!['?'], 4))
            .flatten()
            .collect();
        let groups = repeat_n(self.groups.clone(), 5).flatten().collect();
        Record { cond, groups }
    }
}

fn main() {
    let input = adv2023::read_input();
    let records: Vec<Record> = input.lines().map(|line| line.parse().unwrap()).collect();
    let part1: usize = records.iter().map(|r| r.count_ways()).sum();
    dbg!(part1);

    let records_part2: Vec<Record> = records.iter().map(|r| r.make_into_part2()).collect();
    let part2: usize = records_part2.iter().map(|r| r.count_ways()).sum();
    dbg!(part2);
}
