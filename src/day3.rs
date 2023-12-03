use std::collections::HashMap;

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

#[derive(Debug, Clone)]
struct Grid {
    chars: Vec<Vec<char>>,
    // For part 2, track numbers "attached" to each "*".
    gears: HashMap<(usize, usize), Vec<u32>>,
}

impl Grid {
    fn parse(s: &str) -> Self {
        let chars = s.lines().map(|line| line.chars().collect()).collect();
        let gears = HashMap::new();
        Self { chars, gears }
    }

    // Returns the symbol and its position, if any.
    fn get_adjacent_symbol(&self, y: usize, x: usize) -> Option<(char, usize, usize)> {
        let y_from = if y > 0 { y - 1 } else { y };
        let y_to = if (y + 1) < self.chars.len() { y + 1 } else { y };
        let x_from = if x > 0 { x - 1 } else { x };
        let x_to = if (x + 1) < self.chars[0].len() {
            x + 1
        } else {
            x
        };

        for yy in y_from..=y_to {
            for xx in x_from..=x_to {
                let c = self.chars[yy][xx];
                if is_symbol(c) {
                    return Some((c, yy, xx));
                }
            }
        }
        None
    }

    fn maybe_extract_number(&mut self, y: usize, mut x: usize) -> Option<u32> {
        if !self.chars[y][x].is_digit(10) {
            return None;
        }
        let mut tmp = 0u32;
        let mut symbol_pos = None;
        while x < self.chars[0].len() {
            match self.chars[y][x].to_digit(10) {
                Some(d) => {
                    tmp = tmp * 10 + d;
                    self.chars[y][x] = '.';
                    if symbol_pos.is_none() {
                        symbol_pos = self.get_adjacent_symbol(y, x);
                    }
                }
                None => {
                    break;
                }
            }
            x += 1;
        }
        match symbol_pos {
            None => None,
            Some(('*', y, x)) => {
                // dbg!(y, x);
                self.gears.entry((y, x)).and_modify(|v| v.push(tmp)).or_insert(vec![tmp]);
                // dbg!(&self.gears);
                Some(tmp)
            }
            _ => Some(tmp),
        }
    }

    fn sum_part_numbers(&mut self) -> u32 {
        let mut sum = 0u32;
        for y in 0..self.chars.len() {
            for x in 0..self.chars[0].len() {
                if let Some(number) = self.maybe_extract_number(y, x) {
                    sum += number;
                }
            }
        }
        sum
    }

    fn sum_gear_ratios(&self) -> u32 {
        let mut sum = 0u32;
        for values in self.gears.values() {
            assert!(values.len() <= 2);
            if values.len() == 2 {
                sum += values[0] * values[1];
            }
        }
        sum
    }
}

fn main() {
    let input = adv2023::read_input();
    let mut grid = Grid::parse(&input);
    dbg!(grid.sum_part_numbers());
    dbg!(grid.sum_gear_ratios());
}
