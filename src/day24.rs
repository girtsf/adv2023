use std::{ops::RangeInclusive, str::FromStr};

use adv2023::{Pos, Pos3};
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    pos: Pos3,
    vel: Pos3,
}

#[derive(Debug)]
struct Problem {
    test_area: (f64, f64),
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
            .map(|x| x.parse::<f64>().unwrap())
            .collect_tuple()
            .unwrap();
        let test_area = (from, to);

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

impl Hailstone {
    fn find_future_2d_intersection_with(&self, other: &Hailstone) -> Option<(f64, f64)> {
        assert_ne!(self.vel.y, 0);
        let y10 = self.pos.y as f64;
        let y1v = self.vel.y as f64;
        let x10 = self.pos.x as f64;
        let x1v = self.vel.x as f64;
        let y20 = other.pos.y as f64;
        let y2v = other.vel.y as f64;
        let x20 = other.pos.x as f64;
        let x2v = other.vel.x as f64;

        let denom = y2v - (y1v * x2v / x1v);
        if denom.abs() < 0.0000001 {
            // parallel?
            return None;
        }

        let t2 = (y10 + y1v * ((x20 - x10) / x1v) - y20) / denom;
        if t2 < 0.0 {
            return None;
        }
        let t1 = (x20 + x2v * t2 - x10) / x1v;
        if t1 < 0.0 {
            return None;
        }
        let x = x20 + x2v * t2;
        let y = y20 + y2v * t2;
        // (x, y)
        Some((x, y))
    }
}

impl Problem {
    fn part1_count_intersections(&self) -> usize {
        let mut count = 0usize;
        for i in 0..self.hailstones.len() {
            for j in (i + 1)..self.hailstones.len() {
                let res1 = self.hailstones[i].find_future_2d_intersection_with(&self.hailstones[j]);
                // let res2 = self.hailstones[j].find_future_2d_intersection_with(&self.hailstones[i]);
                // assert_eq!(res1.is_some(), res2.is_some());

                if let Some((x, y)) = res1 {
                    if x >= self.test_area.0
                        && x <= self.test_area.1
                        && y >= self.test_area.0
                        && y <= self.test_area.1
                    {
                        // dbg!(i, j, x, y);
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

fn main() {
    let input = adv2023::read_input();
    let problem: Problem = input.parse().unwrap();
    dbg!(&problem);
    dbg!(&problem.part1_count_intersections());
}
