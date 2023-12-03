use std::collections::HashMap;

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Vec<Vec<(i32, String)>>> {
    input
        .lines()
        .map(|line| {
            let (_, wo_pref) = line.split_once(": ").unwrap();
            let game_turns: Vec<Vec<(i32, String)>> = wo_pref
                .split("; ")
                .map(|turn| {
                    turn.split(", ")
                        .map(|spec| {
                            let (n, color) = spec.split_once(" ").unwrap();
                            (
                                n.parse::<i32>().context(wo_pref.to_string()).unwrap(),
                                color.to_string(),
                            )
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            game_turns
        })
        .collect::<Vec<_>>()
}

fn possible(game: &[Vec<(i32, String)>], bag: &HashMap<&str, i32>) -> bool {
    game.iter().all(|handful| {
        handful
            .iter()
            .all(|(n, color)| *n <= *bag.get(color.as_str()).unwrap())
    })
}

#[aoc(day2, part1)]
fn part1(input: &[Vec<Vec<(i32, String)>>]) -> usize {
    let bag = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    input
        .iter()
        .enumerate()
        .filter_map(|(i, game)| possible(game, &bag).then_some(i + 1))
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[Vec<Vec<(i32, String)>>]) -> i32 {
    input
        .iter()
        .map(|game| {
            let grouped = game
                .iter()
                .flatten()
                .into_grouping_map_by(|(_count, color)| color)
                .max_by_key(|_k, (v, _color)| *v);
            grouped
                .into_iter()
                .map(|(_key, (n, _color))| n)
                .product::<i32>()
        })
        .sum()
}
