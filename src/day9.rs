use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[Vec<i64>]) -> i64 {
    input
        .into_iter()
        .map(|line| {
            let mut lines = vec![line.clone()];
            while lines.last().unwrap().iter().any(|n| *n != 0) {
                let line = lines.last().unwrap();
                lines.push(
                    line.into_iter()
                        .tuple_windows::<(_, _)>()
                        .map(|(a, b)| b - a)
                        .collect(),
                );
            }
            lines.reverse();

            lines[0].push(0);
            for i in 1..lines.len() {
                let delta = *lines[i - 1].last().unwrap();
                let next = *lines[i].last().unwrap() + delta;
                lines[i].push(next);
            }
            *lines.last().unwrap().last().unwrap()
        })
        .sum()
}

#[aoc(day9, part2)]
fn part2(input: &[Vec<i64>]) -> i64 {
    input
        .into_iter()
        .map(|line| {
            let mut lines = vec![line.clone()];
            while lines.last().unwrap().iter().any(|n| *n != 0) {
                let line = lines.last().unwrap();
                lines.push(
                    line.into_iter()
                        .tuple_windows::<(_, _)>()
                        .map(|(a, b)| b - a)
                        .collect(),
                );
            }
            lines.reverse();
            lines.iter_mut().for_each(|line| line.reverse());

            lines[0].push(0);
            for i in 1..lines.len() {
                let delta = *lines[i - 1].last().unwrap();
                let next = *lines[i].last().unwrap() - delta;
                lines[i].push(next);
            }
            *lines.last().unwrap().last().unwrap()
        })
        .sum()
}
