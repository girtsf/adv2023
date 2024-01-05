use itertools::Itertools;
use std::{collections::HashMap, ops::RangeInclusive, str::FromStr};

use adv2023::Pos3;

type BrickIdx = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Brick {
    x: RangeInclusive<isize>,
    y: RangeInclusive<isize>,
    z: RangeInclusive<isize>,
    deleted: bool,
}

#[derive(Debug, Clone)]
struct World {
    bricks: Vec<Brick>,
    pos_to_brick: HashMap<Pos3, BrickIdx>,
}

struct BrickIter {
    brick: Brick,
    x: isize,
    y: isize,
    z: isize,
    done: bool,
}

impl BrickIter {
    fn new(brick: &Brick) -> Self {
        Self {
            brick: brick.clone(),
            x: *brick.x.start(),
            y: *brick.y.start(),
            z: *brick.z.start(),
            done: false,
        }
    }
}

impl Iterator for BrickIter {
    type Item = Pos3;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let item = Some(Pos3::new(self.x, self.y, self.z));
        self.x += 1;
        if self.x > *self.brick.x.end() {
            self.x = *self.brick.x.start();
            self.y += 1;
        }
        if self.y > *self.brick.y.end() {
            self.y = *self.brick.y.start();
            self.z += 1;
        }
        if self.z > *self.brick.z.end() {
            self.done = true;
        }
        item
    }
}

impl FromStr for Brick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 1,0,1~1,2,1
        let (lhs, rhs) = s.split_once('~').unwrap();
        let (x1, y1, z1) = lhs
            .split(',')
            .map(|x| x.parse::<isize>().unwrap())
            .collect_tuple()
            .unwrap();
        let (x2, y2, z2) = rhs
            .split(',')
            .map(|x| x.parse::<isize>().unwrap())
            .collect_tuple()
            .unwrap();
        // let (x2, y2, z2) = rhs.split(',').collect_tuple().unwrap();
        Ok(Brick {
            x: x1.min(x2)..=x1.max(x2),
            y: y1.min(y2)..=y1.max(y2),
            z: z1.min(z2)..=z1.max(z2),
            deleted: false,
        })
    }
}

impl Brick {
    fn blocks_iter(&self) -> BrickIter {
        BrickIter::new(self)
    }
}

impl FromStr for World {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bricks: Vec<Brick> = s
            .lines()
            .map(|line| line.parse::<Brick>().unwrap())
            .collect();
        let mut pos_to_brick = HashMap::new();
        for (i, b) in bricks.iter().enumerate() {
            for pos in b.blocks_iter() {
                pos_to_brick.insert(pos, i);
            }
        }

        Ok(Self {
            bricks,
            pos_to_brick,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let brick: Brick = "0,0,0~1,2,1".parse().unwrap();
        let t: Vec<_> = brick.blocks_iter().collect();
        assert_eq!(
            t,
            vec![
                Pos3 { x: 0, y: 0, z: 0 },
                Pos3 { x: 1, y: 0, z: 0 },
                Pos3 { x: 0, y: 1, z: 0 },
                Pos3 { x: 1, y: 1, z: 0 },
                Pos3 { x: 0, y: 2, z: 0 },
                Pos3 { x: 1, y: 2, z: 0 },
                Pos3 { x: 0, y: 0, z: 1 },
                Pos3 { x: 1, y: 0, z: 1 },
                Pos3 { x: 0, y: 1, z: 1 },
                Pos3 { x: 1, y: 1, z: 1 },
                Pos3 { x: 0, y: 2, z: 1 },
                Pos3 { x: 1, y: 2, z: 1 },
            ]
        );
    }
}

impl World {
    /// Tries dropping a brick, returns true if dropped at all.
    fn try_drop_brick(&mut self, idx: BrickIdx) -> bool {
        let mut dropped = false;

        loop {
            for pos in self.bricks[idx].blocks_iter() {
                if pos.z == 1 {
                    return dropped;
                }
                if let Some(brick_below_idx) = self.pos_to_brick.get(&pos.below()) {
                    if *brick_below_idx != idx {
                        return dropped;
                    }
                }
            }
            for pos in self.bricks[idx].blocks_iter() {
                assert!(self.pos_to_brick.remove(&pos).is_some());
            }
            self.bricks[idx].z =
                (*self.bricks[idx].z.start() - 1)..=(*self.bricks[idx].z.end() - 1);
            for pos in self.bricks[idx].blocks_iter() {
                assert!(self.pos_to_brick.insert(pos, idx).is_none());
            }

            dropped = true;
        }
    }

    fn try_drop_all_bricks(&mut self) -> bool {
        let mut dropped = false;
        for idx in 0..self.bricks.len() {
            if !self.bricks[idx].deleted {
                dropped |= self.try_drop_brick(idx);
            }
        }
        dropped
    }

    fn drop_until_settles(&mut self) {
        while self.try_drop_all_bricks() {}
    }

    fn delete_brick(&mut self, idx: usize) {
        for pos in self.bricks[idx].blocks_iter() {
            assert!(self.pos_to_brick.remove(&pos).is_some());
        }
        self.bricks[idx].deleted = true;
    }

    fn part1_count_disintegratable_bricks(&self) -> usize {
        let mut count = 0usize;

        for idx in 0..self.bricks.len() {
            let mut tmp_world = self.clone();
            tmp_world.delete_brick(idx);
            if !tmp_world.try_drop_all_bricks() {
                count += 1;
            }
        }
        count
    }

    fn part2_count_fallen_bricks(&self) -> usize {
        let mut count = 0usize;

        for idx in 0..self.bricks.len() {
            let mut tmp_world = self.clone();
            tmp_world.delete_brick(idx);
            tmp_world.drop_until_settles();

            for idx2 in 0..self.bricks.len() {
                if idx2 == idx {
                    continue;
                }
                if self.bricks[idx2] != tmp_world.bricks[idx2] {
                    count += 1;
                }
            }
        }
        count
    }
}

fn main() {
    let input = adv2023::read_input();
    let mut world: World = input.parse().unwrap();
    // dbg!(&world.try_drop_brick(2));
    world.drop_until_settles();
    dbg!(world.part1_count_disintegratable_bricks());
    dbg!(world.part2_count_fallen_bricks());
}
