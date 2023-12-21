use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    io::{stderr, stdout, Write},
    ops::RangeInclusive,
    str::FromStr,
};

use adv2023::Pos;
use itertools::Itertools;

#[derive(Debug, Default)]
struct Ranges {
    r: Vec<RangeInclusive<isize>>,
}

impl Ranges {
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

impl Map {
    fn new_part1(input: &str) -> Self {
        Self::new(input, parse_part1)
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
        dbg!(Pos::new(min_y, min_x), Pos::new(max_y, max_x))
    }

    fn calculate_area(&mut self) -> usize {
        let mut by_y: BTreeMap<isize, Vec<isize>> = BTreeMap::new();
        self.corners.iter().for_each(|pos| {
            by_y.entry(pos.y).or_default().push(pos.x);
        });
        by_y.iter_mut().for_each(|(_, v)| v.sort());

        let mut ranges = Ranges::default();
        for (y, xes) in &by_y {
            println!("y: {y} ranges: {:?}", &ranges);
            assert!(xes.len() % 2 == 0);
            for (x1, x2) in xes.iter().tuples() {
            println!("  {x1}-{x2}");

            }
        }
        // dbg!(&by_y);
        todo!()
    }

    // fn flood_fill(&mut self, start: Pos) {
    //     let mut to_visit = BTreeSet::<Pos>::from([start]);
    //     while let Some(pos) = to_visit.pop_first() {
    //         for dir in [Pos::left(), Pos::up(), Pos::right(), Pos::down()] {
    //             let pos2 = pos + dir;
    //             if !self.holes.contains(&pos2) {
    //                 to_visit.insert(pos2);
    //                 self.holes.insert(pos2);
    //             }
    //         }
    //     }
    // }

    // fn area(&self) -> usize {
    //     let mut area = 0usize;
    //     let (min_pos, max_pos) = self.minmax();
    //     for y in min_pos.y ..= max_pos.y {
    //         let mut inside = false;
    //         for x in min_pos.x ..= max_pos.x {
    //             let pos = Pos::new(y, x);
    //             let left_pos = Pos::new(y, x - 1);
    //             if self.holes.contains(&pos) {
    //                 area += 1;
    //                 // if !self.holes.contains(&left_pos) {
    //                 //     // We went . -> #
    //                 //     in_wall = true;
    //                 //     if !inside {
    //                 //         inside = true;
    //                 //     }
    //                 // }
    //             } else {
    //                 if self.holes.contains(&left_pos) {
    //                     // We went # -> .
    //                     inside = !inside;
    //                     // if inside {
    //                     //     inside = false;
    //                     // }
    //                 }
    //                 if inside {
    //                     area += 1;
    //                 }
    //             }
    //         }
    //         dbg!(y, area);
    //     }
    //     area
    // }

    fn draw(&self) {
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
        stdout().lock().flush().unwrap();
    }
}

fn main() {
    let input = adv2023::read_input();
    let mut map = Map::new_part1(&input);
    map.draw();
    dbg!(map.calculate_area());
    // map.dig(&input);
    // dbg!(map.holes.len());
    // map.flood_fill(Pos::new(1, 1));
    // dbg!(map.holes.len());
    // dbg!(&map.pos);
    // 36842: That's not the right answer.
}

// @.....@
// .......
// @.@....
// .......
// .......
// @.@.@.@
// .......
// @@..@.@
// .......
// .@....@
//
//
// #######
// #######
// --#####
// ..#####
// ..#####
// +++##--
// #####..
// -###+++
// .######
// .------ on "-", don't keep the outside itself?
//
