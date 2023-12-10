use std::{collections::VecDeque, iter::repeat, ops::Add, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    y: isize,
    x: isize,
}

impl Pos {
    fn new(y: isize, x: isize) -> Self {
        Self { y, x }
    }

    fn up() -> Self {
        Pos::new(-1, 0)
    }

    fn down() -> Self {
        Pos::new(1, 0)
    }

    fn left() -> Self {
        Pos::new(0, -1)
    }

    fn right() -> Self {
        Pos::new(0, 1)
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        }
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<char>>,
    distance: Vec<Vec<isize>>,
    start: Pos,
    size: Pos,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        let size = Pos::new(tiles.len() as isize, tiles[0].len() as isize);
        let mut distance = Vec::new();
        let mut start = None;
        for y in 0..size.y {
            distance.push(Vec::from_iter(repeat(-1).take(tiles[0].len())));
            for x in 0..size.x {
                if tiles[y as usize][x as usize] == 'S' {
                    start = Some(Pos::new(y as isize, x as isize));
                }
            }
        }
        Ok(Self {
            tiles,
            distance,
            start: start.unwrap(),
            size,
        })
    }
}

impl Map {
    fn t(&self, pos: Pos) -> char {
        self.tiles[pos.y as usize][pos.x as usize]
    }

    fn d(&mut self, pos: Pos) -> &mut isize {
        &mut self.distance[pos.y as usize][pos.x as usize]
    }

    fn in_bounds(&self, pos: Pos) -> bool {
        pos.x >= 0 && pos.x < self.size.x && pos.y >= 0 && pos.y < self.size.y
    }

    fn connects(&self, pos: Pos) -> Option<(Pos, Pos)> {
        if !self.in_bounds(pos) {
            return None;
        }
        match self.t(pos) {
            '|' => Some((pos + Pos::up(), pos + Pos::down())),
            '-' => Some((pos + Pos::left(), pos + Pos::right())),
            'L' => Some((pos + Pos::up(), pos + Pos::right())),
            'J' => Some((pos + Pos::up(), pos + Pos::left())),
            '7' => Some((pos + Pos::down(), pos + Pos::left())),
            'F' => Some((pos + Pos::down(), pos + Pos::right())),
            _ => None,
        }
    }

    fn find_path(&mut self) -> isize {
        *self.d(self.start) = 0;
        // see which directions from start we can go.
        let dirs = vec![Pos::left(), Pos::up(), Pos::right(), Pos::down()];
        for d in dirs {
            let pos = self.start + d;
            if let Some(pos_connects) = self.connects(pos) {
                if self.start == pos_connects.0 || self.start == pos_connects.1 {
                    if let Some(dist) = self.explore(pos) {
                        return dist;
                    }
                }
            }
        }
        panic!("no way");
    }

    fn explore(&mut self, pos: Pos) -> Option<isize> {
        let mut todo = VecDeque::<(Pos, isize)>::new();
        todo.push_back((pos, 0));

        while let Some((pos, dist)) = todo.pop_front() {
            // dbg!(pos, dist);
            if !self.in_bounds(pos) {
                continue;
            }
            if pos == self.start {
                // dbg!("back at start!");
                if dist > 1 {
                    return Some((dist + 1) / 2);
                }
                continue;
            }
            let pos_dist = self.d(pos);
            if *pos_dist != -1 {
                // already explored
                continue;
            }
            *pos_dist = dist + 1;
            let (to1, to2) = self.connects(pos).unwrap();
            todo.push_back((to1, dist + 1));
            todo.push_back((to2, dist + 1));
        }

        None
    }
}

fn main() {
    let input = adv2023::read_input();
    let mut map: Map = input.parse().unwrap();
    // dbg!(&map);
    dbg!(map.find_path());
}
