use itertools::Itertools;
use log::debug;
use std::str::FromStr;

const ORDER_PART1: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

const ORDER_PART2: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

fn cmp_cards(a: char, b: char, use_joker: bool) -> std::cmp::Ordering {
    let order = if use_joker {
        &ORDER_PART2
    } else {
        &ORDER_PART1
    };
    let idx_a = order.iter().position(|&x| x == a).unwrap();
    let idx_b = order.iter().position(|&x| x == b).unwrap();
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

fn determine_type_by_counts(cards: &[char]) -> HandType {
    let label_counts = cards.iter().counts();
    let counts_sorted: Vec<_> = label_counts.values().copied().sorted().rev().collect();

    match counts_sorted[0] {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => {
            if counts_sorted.len() > 1 && counts_sorted[1] == 2 {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        }
        2 => {
            if counts_sorted.len() > 1 && counts_sorted[1] == 2 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        }
        1 => HandType::HighCard,
        _ => panic!("unexpected counts: {label_counts:?}"),
    }
}

fn upgrade_with_joker(t: HandType) -> HandType {
    match t {
        HandType::HighCard => HandType::OnePair,
        HandType::OnePair => HandType::ThreeOfAKind,
        HandType::TwoPair => HandType::FullHouse,
        HandType::ThreeOfAKind => HandType::FourOfAKind,
        HandType::FourOfAKind => HandType::FiveOfAKind,
        _ => panic!("cannot upgrade {t:?}"),
    }
}

impl Hand {
    fn hand_type(&self, use_joker: bool) -> HandType {
        if use_joker {
            let cards_without_jokers: Vec<_> =
                self.cards.iter().copied().filter(|&c| c != 'J').collect();
            let joker_count = 5 - cards_without_jokers.len();
            if joker_count == 5 {
                return HandType::FiveOfAKind;
            }
            let mut t = determine_type_by_counts(&cards_without_jokers);
            for _ in 0..joker_count {
                t = upgrade_with_joker(t);
            }
            t
        } else {
            determine_type_by_counts(&self.cards)
        }
    }
}

fn cmp_hands(a: &Hand, b: &Hand, use_joker: bool) -> std::cmp::Ordering {
    let hand_type_a = a.hand_type(use_joker);
    let hand_type_b = b.hand_type(use_joker);
    if hand_type_a != hand_type_b {
        return hand_type_a.cmp(&hand_type_b);
    }
    // Otherwise, compare each card.
    for i in 0..5 {
        let ord = cmp_cards(a.cards[i], b.cards[i], use_joker);
        if !matches!(ord, std::cmp::Ordering::Equal) {
            return ord;
        }
    }
    panic!("same hands??");
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
    problem.hands.iter().for_each(|hwb| {
        let hand_type1 = hwb.hand.hand_type(false);
        let hand_type2 = hwb.hand.hand_type(true);
        debug!(
            "hand: {} type: {hand_type1:?} type w/ joker: {hand_type2:?}",
            hwb.hand
        );
    });

    // Part 1:
    problem
        .hands
        .sort_by(|a, b| cmp_hands(&a.hand, &b.hand, false));
    dbg!(problem.total_winnings());

    // Part 2:
    problem
        .hands
        .sort_by(|a, b| cmp_hands(&a.hand, &b.hand, true));
    dbg!(problem.total_winnings());
}
