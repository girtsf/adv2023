use std::collections::HashMap;

fn hash(s: &str) -> u8 {
    let mut v = 0usize;
    for c in s.chars() {
        let c8: u8 = c.try_into().unwrap();
        v += c8 as usize;
        // v += c. as_ascii().unwrap().to_u8() as usize;
        v *= 17;
        v %= 256;
    }
    v as u8
}

fn part2(s: &str) -> usize {
    let mut hashmap = HashMap::<u8, Vec<(String, u8)>>::new();
    s.trim().split(',').for_each(|op| {
        if op.ends_with('-') {
            let label = op.trim_end_matches('-');
            let h = hash(label);
            if let Some(v) = hashmap.get_mut(&h) {
                v.retain(|(s, _)| s != label);
            }
        } else {
            let (label, lens_str) = op.split_once('=').unwrap();
            let lens: u8 = lens_str.parse().unwrap();
            let h = hash(label);
            let v = hashmap.entry(h).or_insert(vec![]);
            for (s, prev_lens) in v.iter_mut() {
                if s == label {
                    *prev_lens = lens;
                    return;
                }
            }
            v.push((label.to_string(), lens));
        }
    });

    dbg!(&hashmap);
    let mut power = 0usize;
    for h in 0u8..=255 {
        if let Some(v) = hashmap.get(&h) {
            for (slot, (_, lens)) in v.iter().enumerate() {
                power += (h as usize + 1) * (slot + 1) * (*lens as usize);
            }
        }
    }
    power
}

fn main() {
    let input = adv2023::read_input();
    let part1: usize = input
        .trim()
        .split(',')
        .map(|x| dbg!(hash(x) as usize))
        .sum();
    dbg!(&part1);
    dbg!(part2(&input));
}
