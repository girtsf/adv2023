fn hash(s: &str) -> usize {
    let mut v = 0;
    for c in s.chars() {
        let c8 : u8 = c.try_into().unwrap();
        v += c8 as usize;
        // v += c. as_ascii().unwrap().to_u8() as usize;
        v *= 17;
        v %= 256;
    }
    v
}

fn main() {
    let input = adv2023::read_input();
    let part1: usize = input.trim().split(',').map(|x| dbg!(hash(x))).sum();
    dbg!(&part1);
}
