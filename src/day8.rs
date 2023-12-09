use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use num::integer::lcm;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
enum RL {
    R,
    L,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
struct Line {
    lhs: String,
    rhs: (String, String),
}

struct Input {
    rls: Vec<RL>,
    lines: Vec<Line>,
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Input {
    let (rls, lines) = input.trim().split_once("\n\n").unwrap();
    let rls = rls
        .chars()
        .map(|c| match c {
            'R' => RL::R,
            'L' => RL::L,
            c => panic!("{c}"),
        })
        .collect_vec();
    let lines = lines
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once(" = ").unwrap();
            let (fst, snd) = rhs.split_once(", ").unwrap();
            let fst = &fst[1..];
            let snd = &snd[..snd.len() - 1];
            Line {
                lhs: lhs.to_string(),
                rhs: (fst.to_string(), snd.to_string()),
            }
        })
        .collect_vec();
    Input { rls, lines }
}

#[aoc(day8, part1)]
fn part1(input: &Input) -> i64 {
    let Input { rls, lines } = input;
    let linesmap = lines
        .into_iter()
        .map(|Line { lhs, rhs }| (lhs.clone(), rhs.clone()))
        .collect::<HashMap<_, _>>();
    let mut rls = rls.iter().cycle();

    let mut curr = "AAA";
    let mut steps = 0i64;
    while curr != "ZZZ" {
        steps += 1;
        let (l, r) = linesmap.get(curr).unwrap();
        curr = match rls.next().unwrap() {
            RL::R => r.as_str(),
            RL::L => l.as_str(),
        };
    }

    steps
}

struct CycleData {
    seen_zs: HashMap<String, usize>,
}

#[aoc(day8, part2)]
fn part2(input: &Input) -> usize {
    let Input { rls, lines } = input;
    let linesmap = lines
        .into_iter()
        .map(|Line { lhs, rhs }| (lhs.clone(), rhs.clone()))
        .collect::<HashMap<_, _>>();

    let cycle_data = lines
        .into_iter()
        .filter_map(|Line { lhs, rhs: _ }| lhs.ends_with('A').then_some(lhs.as_str()))
        .map(|start| {
            let mut rls = rls
                .iter()
                .enumerate()
                .cycle()
                .enumerate()
                .map(|(n, x)| (n + 1, x));
            let mut seen = HashMap::new();
            let mut node = start;

            loop {
                let (step, rl) = rls.next().unwrap();
                let (l, r) = linesmap.get(node).unwrap();
                node = match rl.1 {
                    RL::R => r.as_str(),
                    RL::L => l.as_str(),
                };

                if let Some(prev_step) = seen.insert((node, rl.0), step) {
                    seen.insert((node, rl.0), prev_step);
                    return CycleData {
                        seen_zs: seen
                            .into_iter()
                            .filter_map(|((k, _), v)| k.ends_with('Z').then(|| (k.to_string(), v)))
                            .collect(),
                    };
                }
            }
        })
        .collect::<Vec<_>>();
    cycle_data
        .iter()
        .map(|CycleData { seen_zs }| seen_zs.values().copied().next().unwrap())
        .fold(1usize, |x, y| lcm(x, y))
}
