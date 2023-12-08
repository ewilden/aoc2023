use std::cmp::Reverse;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Num(Reverse<i32>),
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        use Card::*;
        match value {
            'A' => Ace,
            'K' => King,
            'Q' => Queen,
            'J' => Jack,
            'T' => Ten,
            n => Num(Reverse(n.to_string().parse().unwrap())),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Card2 {
    Ace,
    King,
    Queen,
    Ten,
    Num(Reverse<i32>),
    Joker,
}

impl From<char> for Card2 {
    fn from(value: char) -> Self {
        use Card2::*;
        match value {
            'A' => Ace,
            'K' => King,
            'Q' => Queen,
            'J' => Joker,
            'T' => Ten,
            n => Num(Reverse(n.to_string().parse().unwrap())),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum HandType {
    FiveK,
    FourK,
    FullHouse,
    ThreeK,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn ty(&self) -> HandType {
        let Hand { cards } = self;
        if cards.iter().all_equal() {
            return HandType::FiveK;
        }
        let counts = cards.iter().copied().counts();
        let counts_counts = counts.values().copied().counts();

        if counts_counts.get(&4).is_some() {
            return HandType::FourK;
        }

        if counts_counts.get(&3).is_some() {
            if counts_counts.get(&2).is_some() {
                return HandType::FullHouse;
            } else {
                return HandType::ThreeK;
            }
        }

        match counts_counts.get(&2).copied().unwrap_or(0) {
            2 => return HandType::TwoPair,
            1 => return HandType::OnePair,
            _ => return HandType::HighCard,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Hand2 {
    cards: [Card2; 5],
}

impl Hand2 {
    fn ty(&self) -> HandType {
        let Hand2 { cards } = self;
        let mut counts = cards.iter().copied().counts();
        let jokers = counts.remove(&Card2::Joker).unwrap_or(0);
        let counts = counts;

        if counts.len() == 0 {
            return HandType::FiveK;
        }

        if counts.values().copied().max().unwrap() + jokers == 5 {
            return HandType::FiveK;
        }

        if counts.values().copied().max().unwrap() + jokers == 4 {
            return HandType::FourK;
        }

        let counts_counts = counts.values().copied().counts();

        // At this point, we know that we have at most two jokers (because three
        // would allow for a four-of-a-kind guaranteed).
        if jokers == 2 {
            // We must have all distinct non-jokers (or else we'd have a four-of-a-kind).
            // Then the best we can do is three-of-a-kind.
            return HandType::ThreeK;
        } else if jokers == 1 {
            // There must be no three-of-a-kind or we'd have made a four-of-a-kind.
            // So we just need to check how many pairs we have (0, 1, 2).
            match counts_counts.get(&2).copied().unwrap_or(0) {
                2 => return HandType::FullHouse,
                1 => return HandType::ThreeK,
                0 | _ => return HandType::OnePair,
            }
        } else {
            assert_eq!(jokers, 0);

            // Normal hand rules. To be lazy just copy-pasting above section.
            if counts_counts.get(&4).is_some() {
                return HandType::FourK;
            }

            if counts_counts.get(&3).is_some() {
                if counts_counts.get(&2).is_some() {
                    return HandType::FullHouse;
                } else {
                    return HandType::ThreeK;
                }
            }

            match counts_counts.get(&2).copied().unwrap_or(0) {
                2 => return HandType::TwoPair,
                1 => return HandType::OnePair,
                _ => return HandType::HighCard,
            }
        }
    }
}

#[aoc_generator(day7)]
fn parse(input: &str) -> String {
    input.to_string()
}

fn parse1(input: &str) -> Vec<(Hand, i64)> {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();
            let hand = hand.chars().map(Card::from).collect::<Vec<_>>();
            let bid = bid.parse::<i64>().unwrap();
            (
                Hand {
                    cards: hand.try_into().unwrap(),
                },
                bid,
            )
        })
        .collect()
}

fn parse2(input: &str) -> Vec<(Hand2, i64)> {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();
            let hand = hand.chars().map(Card2::from).collect::<Vec<_>>();
            let bid = bid.parse::<i64>().unwrap();
            (
                Hand2 {
                    cards: hand.try_into().unwrap(),
                },
                bid,
            )
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &str) -> i64 {
    let mut input = parse1(input);
    input.sort_by(|(hand_a, _), (hand_b, _)| {
        hand_a
            .ty()
            .cmp(&hand_b.ty())
            .then(hand_a.cmp(hand_b))
            .reverse()
    });
    input
        .into_iter()
        .enumerate()
        .map(|(index, (_, rank))| i64::try_from(index + 1).unwrap() * rank)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &str) -> i64 {
    let mut input = parse2(input);
    input.sort_by(|(hand_a, _), (hand_b, _)| {
        hand_a
            .ty()
            .cmp(&hand_b.ty())
            .then(hand_a.cmp(hand_b))
            .reverse()
    });
    input
        .into_iter()
        .enumerate()
        .map(|(index, (_, rank))| i64::try_from(index + 1).unwrap() * rank)
        .sum()
}
