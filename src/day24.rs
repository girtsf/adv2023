use std::{ops::RangeInclusive, str::FromStr};

use adv2023::Pos3;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    pos: Pos3,
    vel: Pos3,
}

#[derive(Debug)]
struct Problem {
    test_area: RangeInclusive<isize>,
    hailstones: Vec<Hailstone>,
}

impl FromStr for Hailstone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lhs, rhs) = s.split_once(" @ ").unwrap();

        let pos = Pos3::from_iter(lhs.split(",").map(|x| x.trim().parse::<isize>().unwrap()));
        let vel = Pos3::from_iter(rhs.split(",").map(|x| x.trim().parse::<isize>().unwrap()));

        Ok(Hailstone { pos, vel })
    }
}

impl FromStr for Problem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Note: I added the coordinate limits to input files.
        let (limits, hailstone_lines) = s.trim().split_once("\n\n").unwrap();
        // dbg!(&limits, &hailstone_lines);

        let (from, to) = limits
            .split(' ')
            .map(|x| x.parse::<isize>().unwrap())
            .collect_tuple()
            .unwrap();
        let test_area = from..=to;

        let hailstones: Vec<Hailstone> = hailstone_lines
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();
        Ok(Problem {
            test_area,
            hailstones,
        })
    }
}

fn main() {
    let input = adv2023::read_input();
    let problem: Problem = input.parse().unwrap();
    dbg!(&problem);
}
