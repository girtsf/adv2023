use std::cmp::{max, min};

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Range {
    pub from: isize,
    // non-inclusive
    pub to: isize,
}

impl Range {
    pub fn new(from: isize, to: isize) -> Self {
        Self { from, to }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Ranges(Vec<Range>);

impl Ranges {
    pub fn empty() -> Self {
        Self(vec![])
    }

    pub fn new(from: isize, to: isize) -> Self {
        Self(vec![Range { from, to }])
    }

    pub fn from_slice(arr: &[(isize, isize)]) -> Self {
        Self(arr.iter().map(|&(from, to)| Range::new(from, to)).collect())
    }

    fn merge(&mut self) {
        let mut idx = 0;
        while (idx + 1) < self.0.len() {
            if self.0[idx].to >= self.0[idx + 1].from {
                self.0[idx].to = max(self.0[idx].to, self.0[idx + 1].to);
                self.0.remove(idx + 1);
            } else {
                idx += 1;
            }
        }
    }

    pub fn union(&self, other: &Ranges) -> Ranges {
        let mut new = self.clone();
        new.0.extend(other.0.clone());
        new.0.sort();
        new.merge();
        new
    }

    pub fn intersect_one(&self, one: &Range) -> Ranges {
        // dbg!(self, one);
        let mut out: Vec<Range> = vec![];
        for r in self.0.iter() {
            if r.from >= one.to {
                break;
            }
            if r.to > one.from && one.to > r.from {
                let from = max(r.from, one.from);
                let to = min(r.to, one.to);
                out.push(Range { from, to });
            }
        }
        Ranges(out)
    }

    pub fn len(&self) -> usize {
        let mut sum = 0usize;
        for &Range { from, to } in self.0.iter() {
            sum += (to - from) as usize;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ranges() {
        assert_eq!(
            Ranges::new(1, 5).union(&Ranges::new(1, 5)),
            Ranges::new(1, 5)
        );
        assert_eq!(
            Ranges::new(1, 5).union(&Ranges::new(0, 5)),
            Ranges::new(0, 5)
        );
        assert_eq!(
            Ranges::new(1, 5).union(&Ranges::new(2, 5)),
            Ranges::new(1, 5)
        );
        assert_eq!(
            Ranges::new(1, 5).union(&Ranges::new(0, 1)),
            Ranges::new(0, 5)
        );
        assert_eq!(
            Ranges::new(1, 5).union(&Ranges::new(0, 2)),
            Ranges::new(0, 5)
        );
        assert_eq!(
            Ranges::new(1, 5).union(&Ranges::new(4, 8)),
            Ranges::new(1, 8)
        );
        assert_eq!(
            Ranges::new(1, 5).union(&Ranges::new(5, 8)),
            Ranges::new(1, 8)
        );
        assert_eq!(
            Ranges::new(1, 5).union(&Ranges::new(6, 8)),
            Ranges::from_slice(&[(1, 5), (6, 8)])
        );
        assert_eq!(
            Ranges::new(1, 5)
                .union(&Ranges::new(6, 8))
                .union(&Ranges::new(5, 6)),
            Ranges::new(1, 8)
        );
        assert_eq!(Ranges::new(1, 2).union(&Ranges::new(10, 15)).len(), 6);

        assert_eq!(
            Ranges::new(1, 10).intersect_one(&Range::new(1, 10)),
            Ranges::new(1, 10)
        );
        assert_eq!(
            Ranges::new(1, 10).intersect_one(&Range::new(0, 11)),
            Ranges::new(1, 10)
        );
        assert_eq!(
            Ranges::new(1, 10).intersect_one(&Range::new(1, 5)),
            Ranges::new(1, 5)
        );
        assert_eq!(
            Ranges::new(1, 10).intersect_one(&Range::new(5, 10)),
            Ranges::new(5, 10)
        );
        assert_eq!(
            Ranges::new(1, 10).intersect_one(&Range::new(3, 4)),
            Ranges::new(3, 4)
        );
        assert_eq!(
            Ranges::from_slice(&[(1, 2), (3, 4), (5, 6)]).intersect_one(&Range::new(2, 5)),
            Ranges::new(3, 4)
        );
    }
}
