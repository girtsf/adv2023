use itertools::Itertools;
use std::str::FromStr;

const ORDER: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

fn cmp_cards(a: char, b: char) -> std::cmp::Ordering {
    let idx_a = ORDER.iter().position(|&x| x == a).unwrap();
    let idx_b = ORDER.iter().position(|&x| x == b).unwrap();
    idx_b.cmp(&idx_a)
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [char; 5],
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let counts = self.cards.iter().counts();
        match counts.len() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 => {
                // AAA B C => ThreeOfAKind
                if counts.values().max().unwrap() == &3 {
                    HandType::ThreeOfAKind
                } else {
                    // AA BB C => TwoPair
                    HandType::TwoPair
                }
            }
            2 => {
                // AAAA B => FourOfAKind
                if counts.values().max().unwrap() == &4 {
                    HandType::FourOfAKind
                } else {
                    // AAA BB => FullHouse
                    HandType::FullHouse
                }
            }
            1 => HandType::FiveOfAKind,
            _ => panic!("unexpected counts: {counts:?}"),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let hand_type_self = self.hand_type();
        let hand_type_other = other.hand_type();
        if hand_type_self != hand_type_other {
            return hand_type_self.cmp(&hand_type_other);
        }
        // Otherwise, compare each card.
        for i in 0..5 {
            let ord = cmp_cards(self.cards[i], other.cards[i]);
            if !matches!(ord, std::cmp::Ordering::Equal) {
                return ord;
            }
        }
        panic!("same hands??");
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", &self.cards.iter().join(""))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct HandWithBid {
    hand: Hand,
    bid: u32,
}

impl FromStr for HandWithBid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand_str, bid_str) = s.split_once(' ').unwrap();
        let hand = Hand {
            cards: hand_str.chars().collect::<Vec<_>>().try_into().unwrap(),
        };
        let bid = bid_str.parse().unwrap();

        Ok(Self { hand, bid })
    }
}

#[derive(Debug)]
struct Problem {
    hands: Vec<HandWithBid>,
}

impl FromStr for Problem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hands = s
            .lines()
            .map(|line| line.parse::<HandWithBid>().unwrap())
            .collect();
        Ok(Self { hands })
    }
}

impl Problem {
    fn total_winnings(&self) -> u32 {
        self.hands
            .iter()
            .enumerate()
            .map(|(i, hwb)| (i as u32 + 1) * hwb.bid)
            .sum()
    }
}

fn main() {
    let input = adv2023::read_input();
    let mut problem: Problem = input.parse().unwrap();

    // dbg!(&problem);
    // problem.hands.iter().for_each(|hwb| {
    //     let hand_type = hwb.hand.hand_type();
    //     debug!("hand: {} type: {hand_type:?}", hwb.hand);
    // });

    problem.hands.sort_by(|a, b| a.hand.cmp(&b.hand));

    // debug!("---after sort---");
    // problem.hands.iter().for_each(|hwb| {
    //     let hand_type = hwb.hand.hand_type();
    //     debug!("hand: {} type: {hand_type:?}", hwb.hand);
    // });

    // part 1:
    dbg!(problem.total_winnings());
}
