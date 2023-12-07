use std::{
    collections::{BTreeMap, VecDeque},
    ops::Bound,
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Clone, Debug)]
struct Input {
    seeds: Vec<i64>,
    maps: Vec<Map>,
}

#[derive(Clone, Debug)]
struct Map {
    #[allow(dead_code)]
    left: String,
    #[allow(dead_code)]
    right: String,
    ranges: Vec<Range>,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Range {
    src_start: i64,
    dest_start: i64,
    len: i64,
}

impl Range {
    fn convert(&self, seed: i64) -> Option<i64> {
        let Self {
            src_start,
            dest_start,
            len,
        } = *self;
        (src_start..src_start + len)
            .contains(&seed)
            .then_some(seed - src_start + dest_start)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
struct RangeBounded {
    start: Option<i64>,
    end: Option<i64>,
    displace: i64,
}

impl RangeBounded {
    fn convert_range(&self, seed_start: i64, seed_len: i64) -> Option<(i64, i64)> {
        let Self {
            start,
            end,
            displace,
        } = *self;
        let source_seed_start = seed_start.max(start.unwrap_or(seed_start));
        let source_seed_end = (seed_start + seed_len).min(end.unwrap_or(seed_start + seed_len));
        let converted_seed_len = source_seed_end - source_seed_start;
        (converted_seed_len > 0).then_some((source_seed_start + displace, converted_seed_len))
    }
}

impl From<Range> for RangeBounded {
    fn from(
        Range {
            src_start,
            dest_start,
            len,
        }: Range,
    ) -> Self {
        RangeBounded {
            start: Some(src_start),
            end: Some(src_start + len),
            displace: dest_start - src_start,
        }
    }
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Input {
    let groups = input.lines().group_by(|line| line.is_empty());

    let mut groups = groups
        .into_iter()
        .filter_map(|(empty, group)| (!empty).then_some(group));

    let seeds = groups.next().unwrap().next().unwrap();
    let (_, seeds) = seeds.split_once(": ").unwrap();
    let seeds = seeds
        .split(" ")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let maps = groups
        .map(|mut group| {
            let (to_stmt, _) = group.next().unwrap().split_once(" ").unwrap();
            let (left, right) = to_stmt.split_once("-to-").unwrap();

            let ranges = group
                .map(|nums| {
                    let (dest_start, rest) = nums.split_once(" ").unwrap();
                    let dest_start = dest_start.parse::<i64>().unwrap();
                    let (src_start, len) = rest.split_once(" ").unwrap();
                    let src_start = src_start.parse::<i64>().unwrap();
                    let len = len.parse::<i64>().unwrap();
                    Range {
                        dest_start,
                        src_start,
                        len,
                    }
                })
                .collect::<Vec<_>>();
            Map {
                left: left.to_owned(),
                right: right.to_owned(),
                ranges,
            }
        })
        .collect::<Vec<_>>();

    Input { seeds, maps }
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> i64 {
    let Input { seeds, maps } = input;

    let mut seeds = seeds.clone();
    for map in maps.iter().map(
        |Map {
             left: _,
             right: _,
             ranges,
         }| {
            ranges
                .into_iter()
                .map(
                    |range @ Range {
                         src_start,
                         dest_start: _,
                         len: _,
                     }| { (*src_start, *range) },
                )
                .collect::<BTreeMap<_, _>>()
        },
    ) {
        seeds = seeds
            .into_iter()
            .map(|seed| {
                let Some((_, candidate)) = map.range(..=seed).rev().next() else {
                    return seed;
                };
                candidate.convert(seed).unwrap_or(seed)
            })
            .collect::<Vec<_>>();
    }

    seeds.into_iter().min().unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> i64 {
    let Input { seeds, maps } = input;

    let mut seeds = seeds
        .clone()
        .into_iter()
        .tuples::<(_, _)>()
        .collect::<Vec<_>>();

    for map in maps.iter().map(
        |Map {
             left: _,
             right: _,
             ranges,
         }| {
            ranges
                .into_iter()
                .map(
                    |range @ Range {
                         src_start,
                         dest_start: _,
                         len: _,
                     }| { (*src_start, *range) },
                )
                .collect::<BTreeMap<_, _>>()
        },
    ) {
        seeds = seeds
            .into_iter()
            .flat_map(|(seed_start, seed_len)| {
                let preceding = map
                    .range(..seed_start)
                    .rev()
                    .next()
                    .map(|(start, _)| Bound::Included(*start))
                    .unwrap_or(Bound::Unbounded);
                let succeeding = map
                    .range(seed_start + seed_len..)
                    .next()
                    .map(|(start, _)| Bound::Excluded(*start))
                    .unwrap_or(Bound::Unbounded);

                let mut ranges_to_consider = map
                    .range((preceding, succeeding))
                    .map(|(_, &range)| RangeBounded::from(range))
                    .collect::<VecDeque<_>>();

                if ranges_to_consider.is_empty() {
                    ranges_to_consider.push_back(RangeBounded {
                        start: None,
                        end: None,
                        displace: 0,
                    });
                }

                if let Some(n) = ranges_to_consider.front().unwrap().start {
                    ranges_to_consider.push_front(RangeBounded {
                        start: None,
                        end: Some(n),
                        displace: 0,
                    });
                }

                if let Some(n) = ranges_to_consider.back().unwrap().end {
                    ranges_to_consider.push_back(RangeBounded {
                        start: Some(n),
                        end: None,
                        displace: 0,
                    });
                }

                ranges_to_consider
                    .into_iter()
                    .filter_map(|range| range.convert_range(seed_start, seed_len))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
    }

    seeds.into_iter().map(|(x, _)| x).min().unwrap()
}
