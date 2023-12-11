use std::{collections::HashSet, str::FromStr};

use adv2023::Pos;

#[derive(Debug)]
struct Image {
    galaxies: Vec<Pos>,
    empty_rows: HashSet<isize>,
    empty_cols: HashSet<isize>,
}

impl FromStr for Image {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut galaxies = vec![];
        let lines: Vec<&str> = s.lines().collect();
        let mut empty_rows = HashSet::from_iter(0..(lines.len() as isize));
        let mut empty_cols = HashSet::from_iter(0..(lines[0].len() as isize));
        lines.iter().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    galaxies.push(Pos::new(y, x));
                    empty_rows.remove(&(y as isize));
                    empty_cols.remove(&(x as isize));
                }
            });
        });
        Ok(Image {
            galaxies,
            empty_rows,
            empty_cols,
        })
    }
}

impl Image {
    fn pairs_shortest_paths(&self, gap_size: isize) -> isize {
        let mut sum = 0;
        for i in 0..self.galaxies.len() {
            let g1 = self.galaxies[i];
            for j in (i + 1)..self.galaxies.len() {
                let g2 = self.galaxies[j];
                let (y_min, y_max) = if g1.y < g2.y {
                    (g1.y, g2.y)
                } else {
                    (g2.y, g1.y)
                };
                let (x_min, x_max) = if g1.x < g2.x {
                    (g1.x, g2.x)
                } else {
                    (g2.x, g1.x)
                };

                // This could be more efficient, but meh.
                let y_gaps = self
                    .empty_rows
                    .iter()
                    .filter(|&&y| y > y_min && y < y_max)
                    .count() as isize;
                let x_gaps = self
                    .empty_cols
                    .iter()
                    .filter(|&&x| x > x_min && x < x_max)
                    .count() as isize;

                let path = (y_max - y_min) + (x_max - x_min) + (gap_size - 1) * (y_gaps + x_gaps);
                sum += path;
            }
        }
        sum
    }
}

fn main() {
    let input = adv2023::read_input();
    let image: Image = input.parse().unwrap();
    dbg!(&image);
    dbg!(image.pairs_shortest_paths(1));
    dbg!(image.pairs_shortest_paths(1_000_000));
}
