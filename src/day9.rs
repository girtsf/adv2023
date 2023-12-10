use std::str::FromStr;

#[derive(Debug, Clone)]
struct Seq(Vec<i64>);

impl FromStr for Seq {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.split(' ').map(|x| x.parse::<i64>().unwrap()).collect();
        Ok(Self(values))
    }
}

impl Seq {
    fn all_zeros(&self) -> bool {
        self.0.iter().all(|&x| x == 0)
    }

    fn deltas(&self) -> Seq {
        let mut out: Vec<i64> = vec![];
        for i in 0..(self.0.len() - 1) {
            out.push(self.0[i + 1] - self.0[i]);
        }
        Seq(out)
    }

    fn deltas_until_zeros(&self) -> Vec<Seq> {
        let mut v = vec![self.clone()];
        while !v.last().unwrap().all_zeros() {
            v.push(v.last().unwrap().deltas());
        }
        v
    }

    fn extrapolate_last(&self) -> i64 {
        self.deltas_until_zeros()
            .iter()
            .rev()
            .fold(0, |acc, s| acc + s.0.last().unwrap())
    }

    fn extrapolate_first(&self) -> i64 {
        self.deltas_until_zeros()
            .iter()
            .rev()
            .fold(0, |acc, s| s.0[0] - acc)
    }
}

fn main() {
    let input = adv2023::read_input();
    let seqs: Vec<Seq> = input.lines().map(|line| line.parse().unwrap()).collect();
    // dbg!(&seqs);
    let part1: i64 = seqs.iter().map(|s| s.extrapolate_last()).sum();
    dbg!(&part1);
    let part2: i64 = seqs.iter().map(|s| s.extrapolate_first()).sum();
    dbg!(&part2);
}
