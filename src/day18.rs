use std::{
    cmp::max,
    collections::{BTreeMap, HashSet},
    io::{stderr, stdout, Write},
};

use adv2023::Pos;
use itertools::Itertools;

// inclusive ranges
#[derive(Debug, Default, Clone)]
struct Ranges(Vec<(isize, isize)>);

impl Ranges {
    fn merge(&mut self) {
        let mut idx = 0;
        while (idx + 1) < self.0.len() {
            if self.0[idx].1 >= self.0[idx + 1].0 {
                self.0[idx].1 = max(self.0[idx].1, self.0[idx + 1].1);
                self.0.remove(idx + 1);
            } else {
                idx += 1;
            }
        }
    }

    fn apply(&mut self, x1: isize, x2: isize) {
        assert!(x1 < x2);
        for idx in 0..self.0.len() {
            let (r1, r2) = self.0[idx];
            // Are both x1 & x2 part of one of the ranges? If so, take it out.
            if x1 >= r1 && x2 <= r2 {
                let mut replacement = vec![];
                if x1 > r1 {
                    replacement.push((r1, x1));
                }
                if r2 > x2 {
                    replacement.push((x2, r2));
                }
                self.0.splice(idx..=idx, replacement);
                return;
            }
        }
        // If we are here, it wasn't part of any of the ranges, so add it.
        self.0.push((x1, x2));
        self.0.sort();
        self.merge();
    }

    fn union(&self, other: &Ranges) -> Ranges {
        let mut new = self.clone();
        new.0.extend(other.0.clone());
        new.0.sort();
        new.merge();
        new
    }

    fn len(&self) -> usize {
        let mut sum = 0usize;
        for &(x1, x2) in self.0.iter() {
            sum += (x2 - x1) as usize + 1;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ranges() {
        let mut r = Ranges::default();
        r.apply(1, 5);
        assert_eq!(r.0, vec![(1, 5)]);
        r.apply(-3, 0);
        assert_eq!(r.0, vec![(-3, 0), (1, 5)]);
        r.apply(0, 1);
        assert_eq!(r.0, vec![(-3, 5)]);
        r.apply(5, 8);
        assert_eq!(r.0, vec![(-3, 8)]);
        r.apply(7, 8);
        assert_eq!(r.0, vec![(-3, 7)]);
        r.apply(1, 3);
        assert_eq!(r.0, vec![(-3, 1), (3, 7)]);
        assert_eq!(r.len(), 10);

        assert_eq!(
            Ranges(vec![(0, 3)]).union(&Ranges(vec![(1, 2)])).0,
            vec![(0, 3)]
        );
        assert_eq!(
            Ranges(vec![(1, 2)]).union(&Ranges(vec![(2, 3)])).0,
            vec![(1, 3)]
        );
        assert_eq!(
            Ranges(vec![(1, 3)]).union(&Ranges(vec![(2, 4)])).0,
            vec![(1, 4)]
        );
    }
}

#[derive(Debug, Default)]
struct Map {
    corners: HashSet<Pos>,

    // For initial digging.
    pos: Pos,
}

fn parse_part1(line: &str) -> Pos {
    let items: Vec<_> = line.split(' ').collect();
    let dir_c = items[0].chars().next().unwrap();
    let count: isize = items[1].parse().unwrap();
    let mut dir = match dir_c {
        'R' => Pos::right(),
        'L' => Pos::left(),
        'U' => Pos::up(),
        'D' => Pos::down(),
        _ => panic!(),
    };
    dir.x *= count;
    dir.y *= count;
    dir
}

fn parse_part2(line: &str) -> Pos {
    let hex_str = line.split_once('#').unwrap().1.trim_end_matches(')');
    assert_eq!(hex_str.len(), 6);
    let count = isize::from_str_radix(&hex_str[0..5], 16).unwrap();
    let mut dir = match hex_str.chars().nth(5).unwrap() {
        '0' => Pos::right(),
        '1' => Pos::down(),
        '2' => Pos::left(),
        '3' => Pos::up(),
        _ => panic!(),
    };
    dir.x *= count;
    dir.y *= count;
    dir
}

impl Map {
    fn new_part1(input: &str) -> Self {
        Self::new(input, parse_part1)
    }

    fn new_part2(input: &str) -> Self {
        Self::new(input, parse_part2)
    }

    fn new(input: &str, line_parser: fn(&str) -> Pos) -> Self {
        let mut map = Self::default();
        map.corners.insert(map.pos);
        input
            .lines()
            .for_each(|line| map.parse_and_execute(line, line_parser));
        map
    }

    fn parse_and_execute(&mut self, line: &str, line_parser: fn(&str) -> Pos) {
        let move_pos = line_parser(line);
        let new_pos = self.pos + move_pos;
        if !self.corners.insert(new_pos) {
            assert_eq!(new_pos, Pos::default());
        }
        self.pos = new_pos;
        // dbg!(pos);
    }

    fn minmax(&self) -> (Pos, Pos) {
        let (min_y, max_y) = self
            .corners
            .iter()
            .map(|pos| pos.y)
            .minmax()
            .into_option()
            .unwrap();
        let (min_x, max_x) = self
            .corners
            .iter()
            .map(|pos| pos.x)
            .minmax()
            .into_option()
            .unwrap();
        (Pos::new(min_y, min_x), Pos::new(max_y, max_x))
    }

    fn calculate_area(&mut self) -> usize {
        let mut by_y: BTreeMap<isize, Vec<isize>> = BTreeMap::new();
        self.corners.iter().for_each(|pos| {
            by_y.entry(pos.y).or_default().push(pos.x);
        });
        by_y.iter_mut().for_each(|(_, v)| v.sort());

        let mut area = 0usize;
        let mut ranges = Ranges::default();
        let mut maybe_prev_y = None;
        for (y, xes) in &by_y {
            if let Some(prev_y) = maybe_prev_y {
                let delta_y = y - prev_y - 1;
                let len = ranges.len();
                // println!("delta_y: {delta_y} len: {len}");
                area += delta_y as usize * len;
            }
            // println!("y: {y} ranges: {:?}", &ranges);
            assert!(xes.len() % 2 == 0);
            let mut new_ranges = ranges.clone();
            for (x1, x2) in xes.iter().tuples() {
                // println!("  {x1}-{x2}");
                new_ranges.apply(*x1, *x2);
            }
            // println!("new range: {:?} (len={})", &new_ranges, new_ranges.len());

            area += ranges.union(&new_ranges).len();

            ranges = new_ranges;
            maybe_prev_y = Some(y);
        }
        area
    }

    fn draw(&self) {
        stderr().lock().flush().unwrap();
        println!();
        let (min_pos, max_pos) = self.minmax();
        for y in min_pos.y..=max_pos.y {
            for x in min_pos.x..=max_pos.x {
                let pos = Pos::new(y, x);
                if self.corners.contains(&pos) {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
        stdout().lock().flush().unwrap();
        stderr().lock().flush().unwrap();
    }
}

fn main() {
    let input = adv2023::read_input();

    let mut map = Map::new_part1(&input);
    // map.draw();
    dbg!(map.calculate_area());

    map = Map::new_part2(&input);
    // map.draw();
    dbg!(map.calculate_area());
}
