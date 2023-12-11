use std::collections::{BTreeSet, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("{c}"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[aoc(day11, part1)]
fn part1(input: &[Vec<bool>]) -> usize {
    let mut input = input.to_vec();
    for i in (0..input.len()).rev() {
        if input[i].iter().all(|t| !t) {
            let all_bools = input[i].clone();
            input.insert(i, all_bools);
        }
    }

    for i in (0..input[0].len()).rev() {
        if input.iter().all(|line| !line[i]) {
            for line in input.iter_mut() {
                line.insert(i, false);
            }
        }
    }

    let mapped = input
        .into_iter()
        .map(|line| line.into_iter().enumerate())
        .enumerate()
        .flat_map(|(r, line)| line.flat_map(move |(c, t)| t.then_some((r, c))))
        .collect::<HashSet<_>>();

    mapped
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|((a, b), (c, d))| c.abs_diff(*a) + d.abs_diff(*b))
        .sum::<usize>()
}

#[aoc(day11, part2)]
fn part2(input: &[Vec<bool>]) -> usize {
    let mut big_rows: BTreeSet<usize> = BTreeSet::new();
    for i in (0..input.len()).rev() {
        if input[i].iter().all(|t| !t) {
            big_rows.insert(i);
        }
    }

    let mut big_cols: BTreeSet<usize> = BTreeSet::new();
    for i in (0..input[0].len()).rev() {
        if input.iter().all(|line| !line[i]) {
            big_cols.insert(i);
        }
    }

    let mapped = input
        .into_iter()
        .map(|line| line.into_iter().enumerate())
        .enumerate()
        .flat_map(|(r, line)| line.flat_map(move |(c, t)| t.then_some((r, c))))
        .collect::<HashSet<_>>();

    mapped
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|((a, b), (c, d))| {
            let num_big_rows = big_rows.range(a.min(c)..a.max(c)).count();
            let num_big_cols = big_cols.range(b.min(d)..b.max(d)).count();
            c.abs_diff(*a) + num_big_rows * 1000000 - num_big_rows
                + d.abs_diff(*b)
                + num_big_cols * 1000000
                - num_big_cols
        })
        .sum::<usize>()
}
