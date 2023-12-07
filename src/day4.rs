use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Card {
    winning: Vec<i64>,
    have: Vec<i64>,
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let (_, nums) = line.split_once(": ").unwrap();
            let (winning, have) = nums.split_once(" | ").unwrap();
            let process = |s: &str| {
                s.split_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect::<Vec<_>>()
            };
            Card {
                winning: process(winning),
                have: process(have),
            }
        })
        .collect::<Vec<_>>()
}

#[aoc(day4, part1)]
fn part1(input: &[Card]) -> i64 {
    input
        .into_iter()
        .map(|Card { winning, have }| {
            let winning = winning.iter().collect::<HashSet<_>>();
            let count = have.iter().filter(|n| winning.contains(*n)).count();
            if count >= 1 {
                (2 as i64).pow(u32::try_from(count).unwrap() - 1)
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &[Card]) -> i64 {
    let mut scores = vec![1i64; input.len()];

    for (i, Card { winning, have }) in input.into_iter().rev().enumerate() {
        let winning = winning.iter().collect::<HashSet<_>>();
        let count = have.iter().filter(|n| winning.contains(*n)).count();
        let score: i64 = scores[i - count..i].iter().sum();
        scores[i] += score;
    }

    scores.iter().sum()
}
