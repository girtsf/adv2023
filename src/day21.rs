use std::{collections::HashSet, str::FromStr};

use adv2023::Pos;

#[derive(Debug, Clone)]
struct Map {
    rocks: HashSet<Pos>,
    size: Pos,
    orig_size: Pos,
    start: Pos,
}

type Reachable = HashSet<Pos>;

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        let size = Pos::new(grid.len(), grid[0].len());
        let mut rocks = HashSet::<Pos>::new();
        let mut start = None;
        for y in 0..size.y {
            for x in 0..size.x {
                let pos = Pos::new(y, x);
                match grid[y as usize][x as usize] {
                    'S' => {
                        start = Some(pos);
                    }
                    '#' => {
                        rocks.insert(pos);
                    }
                    '.' => {}
                    _ => panic!(),
                }
            }
        }
        let start = start.unwrap();
        Ok(Map {
            rocks,
            size,
            orig_size: size,
            start,
        })
    }
}

impl Map {
    fn steps(&self, from: &Reachable) -> Reachable {
        let mut next = Reachable::new();
        for pos in from.iter() {
            let poses_maybe = [
                pos + &Pos::up(),
                pos + &Pos::down(),
                pos + &Pos::left(),
                pos + &Pos::right(),
            ];
            for pos2 in poses_maybe {
                if pos2.check_bounds(&self.size) && !self.rocks.contains(&pos2) {
                    next.insert(pos2);
                }
            }
        }
        next
    }

    fn draw(&self, reachable: &Reachable, from: Pos, size: Pos) {
        println!();
        for y in (from.y as usize)..(from.y + size.y) as usize {
            for x in (from.x as usize)..(from.x + size.y) as usize {
                let pos = Pos::new(y, x);
                if self.rocks.contains(&pos) {
                    print!("#");
                } else if pos == self.start {
                    print!("S");
                } else if reachable.contains(&pos) {
                    print!("O");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn expand_right(&mut self, copies: usize) {
        for &Pos { x, y } in self.rocks.clone().iter() {
            for i in 1..=copies {
                self.rocks
                    .insert(Pos::new(y, x + (i as isize) * self.size.x));
            }
        }
        self.size = Pos::new(self.size.y, self.size.x * (1 + copies) as isize);
    }
    fn expand(&mut self, size: isize) {
        assert!(size % 2 == 1);
        let mut new_rocks: Vec<Pos> = vec![];
        for xx in 0..size {
            for yy in 0..size {
                for &Pos { x, y } in self.rocks.iter() {
                    new_rocks.push(Pos::new(y + yy * self.size.y, x + xx * self.size.x));
                }
            }
        }
        self.rocks = HashSet::<Pos>::from_iter(new_rocks);
        self.start = Pos::new(
            self.start.y + self.size.y * (size / 2),
            self.start.x + self.size.x * (size / 2),
        );
        self.size = Pos::new(self.size.y * size, self.size.x * size);
    }

    fn count_area(&self, reachable: &Reachable, y_sq: isize, x_sq: isize) -> usize {
        let y = self.start.y - (self.orig_size.y / 2) + (y_sq * self.orig_size.y);
        let x = self.start.x - (self.orig_size.x / 2) + (x_sq * self.orig_size.x);
        let mut count = 0usize;
        for xx in 0..self.orig_size.x {
            for yy in 0..self.orig_size.y {
                let pos = Pos::new(y + yy, x + xx);
                if reachable.contains(&pos) {
                    count += 1;
                }
            }
        }
        count
    }
}

fn main() {
    let input = adv2023::read_input();
    let mut map: Map = input.parse().unwrap();

    let trial_big_steps = 2;

    map.expand(1 + trial_big_steps * 2);
    let mut reachable = Reachable::from([map.start.clone()]);

    let to = 131 * trial_big_steps + 65;
    for _ in 0..to {
        reachable = map.steps(&reachable);
    }
    dbg!(&reachable.len());
    dbg!(&map.orig_size);
    let starting = map.count_area(&reachable, 0, 0);
    let other = map.count_area(&reachable, 0, 1);
    let other_ = map.count_area(&reachable, 1, 0);
    assert_eq!(other, other_);

    dbg!(starting, other);

    let to_right_3_4 = map.count_area(&reachable, 0, trial_big_steps);
    let to_left_3_4 = map.count_area(&reachable, 0, -trial_big_steps);
    let to_up_3_4 = map.count_area(&reachable, -trial_big_steps, 0);
    let to_down_3_4 = map.count_area(&reachable, trial_big_steps, 0);

    dbg!(to_right_3_4, to_left_3_4, to_up_3_4, to_down_3_4);

    let to_right_up_1_8 = map.count_area(&reachable, -1, trial_big_steps);
    let to_right_up_7_8 = map.count_area(&reachable, -1, trial_big_steps - 1);
    let to_right_up_1_8_ = map.count_area(&reachable, -trial_big_steps, 1);
    assert_eq!(to_right_up_1_8, to_right_up_1_8_);

    let to_left_up_1_8 = map.count_area(&reachable, -1, -trial_big_steps);
    let to_left_up_7_8 = map.count_area(&reachable, -1, -(trial_big_steps - 1));
    let to_left_up_1_8_ = map.count_area(&reachable, -trial_big_steps, -1);
    assert_eq!(to_left_up_1_8, to_left_up_1_8_);

    let to_left_down_1_8 = map.count_area(&reachable, 1, -trial_big_steps);
    let to_left_down_7_8 = map.count_area(&reachable, 1, -(trial_big_steps - 1));
    let to_left_down_1_8_ = map.count_area(&reachable, trial_big_steps, -1);
    assert_eq!(to_left_down_1_8, to_left_down_1_8_);

    let to_right_down_1_8 = map.count_area(&reachable, 1, trial_big_steps);
    let to_right_down_7_8 = map.count_area(&reachable, 1, trial_big_steps - 1);
    let to_right_down_1_8_ = map.count_area(&reachable, trial_big_steps, 1);
    assert_eq!(to_right_down_1_8, to_right_down_1_8_);

    dbg!(
        to_right_up_1_8,
        to_right_up_7_8,
        to_left_up_1_8,
        to_left_up_7_8,
        to_left_down_1_8,
        to_left_down_7_8,
        to_right_down_1_8,
        to_right_down_7_8
    );

    let big_steps = 202300;

    let starting_count = 1 + (4 * (big_steps - 2)) * (big_steps / 2) / 2;
    let other_count = (4 + 4 * (big_steps - 1)) * (big_steps / 2) / 2;
    let corner_7_8_count = big_steps - 1;
    let corner_1_8_count = big_steps;

    dbg!(starting_count);
    dbg!(other_count);
    dbg!(corner_7_8_count);
    dbg!(corner_1_8_count);

    let tot = starting_count * starting
        + other_count * other
        + (to_right_3_4 + to_left_3_4 + to_up_3_4 + to_down_3_4)
        + corner_1_8_count
            * (to_right_up_1_8 + to_left_up_1_8 + to_left_down_1_8 + to_right_down_1_8)
        + corner_7_8_count
            * (to_right_up_7_8 + to_left_up_7_8 + to_left_down_7_8 + to_right_down_7_8);
    dbg!(tot);

    dbg!(&reachable.len());
    // map.draw(&reachable, Pos::new(0, 0), map.size);
    // println!("\n\n");
    // map.draw(
    //     &reachable,
    //     Pos::new(2 * map.orig_size.y, 2 * map.orig_size.x),
    //     map.orig_size,
    // );
}
