const DIGITS: &[(&str, u32)] = &[
    (&"one", 1),
    (&"two", 2),
    (&"three", 3),
    (&"four", 4),
    (&"five", 5),
    (&"six", 6),
    (&"seven", 7),
    (&"eight", 8),
    (&"nine", 9),
];

fn try_parse_digit(s: &str) -> Option<u32> {
    if let Some(digit) = s.chars().next().unwrap().to_digit(10) {
        return Some(digit);
    }
    for (word, value) in DIGITS {
        if s.starts_with(word) {
            return Some(*value);
        }
    }
    None
}

fn find_first_digit(s: &str) -> u32 {
    (0..s.len()).find_map(|i| try_parse_digit(&s[i..])).unwrap()
}

fn find_last_digit(s: &str) -> u32 {
    (0..s.len())
        .rev()
        .find_map(|i| try_parse_digit(&s[i..]))
        .unwrap()
}

fn parse_line_part1(line: &str) -> u32 {
    let first = line.chars().find(|c| c.is_digit(10)).unwrap();
    let last = line.chars().rev().find(|c| c.is_digit(10)).unwrap();
    first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
}

fn parse_line_part2(line: &str) -> u32 {
    let first = find_first_digit(line);
    let last = find_last_digit(line);
    first * 10 + last
}

fn main() {
    let input = adv2023::read_input();
    let part1: u32 = input.lines().map(|line| parse_line_part1(line)).sum();
    dbg!(&part1);
    let part2: u32 = input.lines().map(|line| parse_line_part2(line)).sum();
    dbg!(&part2);
}
