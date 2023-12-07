use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

// tc - c^2 - record
fn quad_roots(t: f64, record: f64) -> [f64; 2] {
    let a = -1f64;
    let b = t;
    let c = -record;

    let p = (-b + f64::sqrt(b.powi(2) - 4f64 * a * c)) / (2f64 * a);
    let q = (-b - f64::sqrt(b.powi(2) - 4f64 * a * c)) / (2f64 * a);
    [p.min(q), p.max(q)]
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<(i64, i64)> {
    let mut input = input.lines().map(|line| {
        line.split_whitespace()
            .skip(1)
            .map(|n| n.parse::<i64>().unwrap())
    });
    let times = input.next().unwrap();
    let distances = input.next().unwrap();
    times.zip(distances).collect::<Vec<_>>()
}

#[aoc(day6, part1)]
fn part1(input: &[(i64, i64)]) -> i64 {
    input
        .into_iter()
        .map(|(time, record)| {
            let [floor, ceil] = quad_roots(time.clone() as f64, record.clone() as f64);
            let floor = floor.floor() as i64 + 1;
            let ceil = ceil.ceil() as i64 - 1;
            ceil - floor + 1
        })
        .product()
}

#[aoc(day6, part2)]
fn part2(input: &[(i64, i64)]) -> i64 {
    let time = input
        .into_iter()
        .map(|(t, _)| t.to_string())
        .join("")
        .parse::<i64>()
        .unwrap();
    let distance = input
        .into_iter()
        .map(|(_, d)| d.to_string())
        .join("")
        .parse::<i64>()
        .unwrap();
    let input = [(time, distance)];
    part1(&input)
}
