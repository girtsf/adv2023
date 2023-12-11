use adv2023::Pos;
use std::collections::VecDeque;
use std::{iter::repeat, str::FromStr};

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

    fn d(&self, pos: Pos) -> isize {
        self.distance[pos.y as usize][pos.x as usize]
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

    fn clear_distance(&mut self) {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                self.distance[y as usize][x as usize] = -1;
            }
        }
    }

    fn find_path(&mut self) -> isize {
        // Try out all possible options for 'S'.
        for s in "|-LJ7F".chars() {
            // dbg!(&s);
            self.tiles[self.start.y as usize][self.start.x as usize] = s;
            self.clear_distance();
            if let Some(dist) = self.explore(self.start) {
                return dist;
            }
        }
        panic!("no way");
    }

    fn have_visited(&self, pos: Pos) -> bool {
        self.d(pos) != -1
    }

    fn explore(&mut self, pos: Pos) -> Option<isize> {
        let mut todo = VecDeque::<(Pos, isize)>::new();
        todo.push_back((pos, 0));

        while let Some((pos, dist)) = todo.pop_front() {
            // println!("pos={pos:?} dist={dist}");
            if self.d(pos) == dist {
                return Some(dist);
            }
            self.distance[pos.y as usize][pos.x as usize] = dist;
            if let Some((to1, to2)) = self.connects(pos) {
                if !self.in_bounds(to1) || !self.in_bounds(to2) {
                    return None;
                }
                if self.d(to1) != (dist - 1) && self.d(to2) != (dist - 1) && dist != 0 {
                    // We didn't come from either of the directions.
                    return None;
                }
                if !self.have_visited(to1) {
                    todo.push_back((to1, dist + 1));
                }
                if !self.have_visited(to2) {
                    todo.push_back((to2, dist + 1));
                }
            } else {
                return None;
            }
        }
        None
    }

    fn count_inside(&self) -> isize {
        let mut count = 0;
        for y in 0..self.size.y {
            // println!(
            //     "y={y} | {} | {}",
            //     join(&self.tiles[y as usize], ""),
            //     join(&self.distance[y as usize], " ")
            // );
            // dbg!(&self.distance[y as usize]);
            let mut inside = false;
            let mut wall_enter = ' ';
            for x in 0..self.size.x {
                if self.distance[y as usize][x as usize] != -1 {
                    let t = self.tiles[y as usize][x as usize];
                    match t {
                        '|' => inside = !inside,
                        'F' | 'L' => {
                            wall_enter = t;
                        }
                        'J' if wall_enter == 'F' => {
                            inside = !inside;
                        }
                        '7' if wall_enter == 'L' => {
                            inside = !inside;
                        }
                        _ => {}
                    }
                } else if inside {
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
    // Part 1:
    dbg!(map.find_path());
    // dbg!(&map);
    // Part 2:
    dbg!(map.count_inside());
}
