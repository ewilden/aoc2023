use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect::<Vec<_>>()
}

fn is_symbol(c: char) -> bool {
    !c.is_numeric() && c != '.'
}

fn neighborhood(
    row: usize,
    (start_col, end_col): (usize, usize),
) -> impl IntoIterator<Item = (usize, usize)> {
    (row.saturating_sub(1)..=row.saturating_add(1))
        .cartesian_product(start_col.saturating_sub(1)..=end_col.saturating_add(1))
}

#[aoc(day3, part1)]
fn part1(input: &[String]) -> i64 {
    let symbol_locs = input
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .flat_map(move |(col, chr)| is_symbol(chr).then_some((row, col)))
        })
        .collect::<HashSet<_>>();
    input
        .iter()
        .enumerate()
        .map(|(row, line)| {
            let regions = line
                .chars()
                .enumerate()
                .map(|(col, chr)| ((col, col), vec![chr]))
                .coalesce(|prev, curr| {
                    if prev.1.iter().all(|c| c.is_numeric())
                        && curr.1.iter().all(|c| c.is_numeric())
                    {
                        Ok((
                            (prev.0 .0, curr.0 .1),
                            prev.1.into_iter().chain(curr.1).collect(),
                        ))
                    } else {
                        Err((prev, curr))
                    }
                })
                .collect::<Vec<_>>();
            let mut sum = 0;
            for ((start, end), chars) in regions {
                if !chars.iter().all(|c| c.is_numeric()) {
                    continue;
                }
                let num = chars
                    .into_iter()
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap();
                if neighborhood(row, (start, end))
                    .into_iter()
                    .any(|loc| symbol_locs.contains(&loc))
                {
                    sum += num;
                }
            }
            sum
        })
        .sum()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Number {
    row: usize,
    start_col: usize,
    end_col: usize,
    num: i64,
}

#[aoc(day3, part2)]
fn part2(input: &[String]) -> i64 {
    let number_regions: HashMap<(usize, usize), Number> = input
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, chr)| ((col, col), vec![chr]))
                .coalesce(|prev, curr| {
                    if prev.1.iter().all(|c| c.is_numeric())
                        && curr.1.iter().all(|c| c.is_numeric())
                    {
                        Ok((
                            (prev.0 .0, curr.0 .1),
                            prev.1.into_iter().chain(curr.1).collect(),
                        ))
                    } else {
                        Err((prev, curr))
                    }
                })
                .flat_map(move |((start_col, end_col), chars)| {
                    chars.iter().all(|c| c.is_numeric()).then(move || Number {
                        row,
                        start_col,
                        end_col,
                        num: chars
                            .into_iter()
                            .collect::<String>()
                            .parse::<i64>()
                            .unwrap(),
                    })
                })
        })
        .flat_map(
            |me @ Number {
                 row,
                 start_col,
                 end_col,
                 num: _,
             }| {
                std::iter::once(row)
                    .cartesian_product(start_col..=end_col)
                    .map(move |k| (k, me))
            },
        )
        .collect::<HashMap<_, _>>();

    input
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            let number_regions = &number_regions;
            line.chars().enumerate().flat_map(move |(col, chr)| {
                if chr != '*' {
                    return None;
                }
                let nums = neighborhood(row, (col, col))
                    .into_iter()
                    .flat_map(|k| number_regions.get(&k))
                    .unique()
                    .map(|Number { num, .. }| num)
                    .collect::<Vec<_>>();
                (nums.len() == 2).then(move || nums.into_iter().product::<i64>())
            })
        })
        .sum()
}
