type Map = Vec<Vec<char>>;

fn parse_map(s: &str) -> Map {
    s.lines().map(|line| line.chars().collect()).collect()
}

fn check_vertical_mirror(map: &Map, col: usize) -> bool {
    for c in 0..=col {
        if (col + 1 + c) >= map[0].len() {
            break;
        }
        for r in 0..map.len() {
            if map[r][col - c] != map[r][col + 1 + c] {
                return false;
            }
        }
    }
    return true;
}

fn find_vertical_line(map: &Map, ignore_col: Option<usize>) -> Option<usize> {
    for col in 0..(map[0].len() - 1) {
        if Some(col) == ignore_col {
            continue;
        }
        if check_vertical_mirror(map, col) {
            return Some(col);
        }
    }
    None
}

fn check_horizontal_mirror(map: &Map, row: usize) -> bool {
    for r in 0..=row {
        if (row + 1 + r) >= map.len() {
            break;
        }
        for c in 0..map[0].len() {
            if map[row - r][c] != map[row + 1 + r][c] {
                return false;
            }
        }
    }
    return true;
}

fn find_horizontal_line(map: &Map, ignore_row: Option<usize>) -> Option<usize> {
    for row in 0..(map.len() - 1) {
        if Some(row) == ignore_row {
            continue;
        }
        if check_horizontal_mirror(map, row) {
            return Some(row);
        }
    }
    None
}

fn part1(m: &Map) -> usize {
    if let Some(col) = find_vertical_line(m, None) {
        return col + 1;
    }
    if let Some(row) = find_horizontal_line(m, None) {
        return (row + 1) * 100;
    }
    panic!()
}

fn flip(c: char) -> char {
    match c {
        '#' => '.',
        '.' => '#',
        _ => panic!(),
    }
}

fn part2(map: &mut Map) -> usize {
    let ignore_col = dbg!(find_vertical_line(map, None));
    let ignore_row = dbg!(find_horizontal_line(map, None));
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            map[r][c] = flip(map[r][c]);
            if let Some(col) = find_vertical_line(map, ignore_col) {
                return col + 1;
            }
            if let Some(row) = find_horizontal_line(map, ignore_row) {
                return (row + 1) * 100;
            }
            map[r][c] = flip(map[r][c]);
        }
    }
    panic!()
}

fn main() {
    let input = adv2023::read_input();
    let mut maps: Vec<Map> = input.split("\n\n").map(|s| parse_map(s)).collect();
    let part1: usize = maps.iter().map(part1).sum();
    dbg!(&part1);
    let part2: usize = maps.iter_mut().map(part2).sum();
    dbg!(&part2);
}
