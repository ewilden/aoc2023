use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<String> {
    input.lines().map(|x| x.to_string()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[String]) -> i32 {
    input
        .into_iter()
        .map(|s| {
            let digits = s.chars().filter(|x| x.is_numeric()).collect::<Vec<_>>();
            let first = *digits.first().unwrap();
            let last = *digits.last().unwrap();
            format!("{first}{last}").parse::<i32>().unwrap()
        })
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &[String]) -> i32 {
    let replacements = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let replacements_reversed = replacements
        .iter()
        .map(|(k, v)| (k.chars().rev().collect::<String>(), v))
        .collect::<HashMap<_, _>>();

    input
        .iter()
        .map(|line| {
            let first = (|| {
                for (i, c) in line.char_indices() {
                    if c.is_numeric() {
                        return format!("{c}");
                    }
                    if let Some(k) = replacements.keys().find(|k| line[..(i + 1)].ends_with(**k)) {
                        return format!("{}", replacements.get(k).unwrap());
                    }
                }
                unreachable!()
            })();
            let line = line.chars().rev().collect::<String>();
            let last = (|| {
                for (i, c) in line.char_indices() {
                    if c.is_numeric() {
                        return format!("{c}");
                    }
                    if let Some(k) = replacements_reversed
                        .keys()
                        .find(|k| line[..(i + 1)].ends_with(&**k))
                    {
                        return format!("{}", replacements_reversed.get(k).unwrap());
                    }
                }
                unreachable!()
            })();
            format!("{first}{last}").parse::<i32>().unwrap()
        })
        .sum()
}
