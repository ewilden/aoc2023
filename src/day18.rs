use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Bound,
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use num_derive::ToPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ToPrimitive)]
enum Dir {
    Right = 0,
    Down,
    Left,
    Up,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Dig {
    dir: Dir,
    dist: i64,
    color: String,
}

impl Dig {
    fn execute(&self, loc: (i64, i64)) -> (i64, i64) {
        let Self {
            dir,
            dist,
            color: _,
        } = self;
        dir.apply(loc, *dist)
    }
}

impl Dir {
    fn apply(&self, (row, col): (i64, i64), n: i64) -> (i64, i64) {
        match self {
            Dir::Up => (row - n, col),
            Dir::Right => (row, col + n),
            Dir::Down => (row + n, col),
            Dir::Left => (row, col - n),
        }
    }
}

#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<Dig> {
    input
        .lines()
        .map(|line| {
            let (dir, rest) = line.split_once(" ").unwrap();
            let dir = match dir {
                "R" => Dir::Right,
                "D" => Dir::Down,
                "U" => Dir::Up,
                "L" => Dir::Left,
                _ => panic!("{dir}"),
            };
            let (dist, color) = rest.split_once(" ").unwrap();
            let dist = dist.parse::<i64>().unwrap();
            let color = color.strip_prefix("(#").unwrap().strip_suffix(')').unwrap();
            Dig {
                dir,
                dist,
                color: color.to_owned(),
            }
        })
        .collect_vec()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ColDir {
    IncreasingAkaSouth,
    DecreasingAkaNorth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Turn {
    Clockwise,
    CounterClockwise,
}

#[aoc(day18, part1)]
fn part1(input: &[Dig]) -> i64 {
    let digs = {
        let mut loc = (0i64, 0i64);
        let mut digs = Vec::new();
        for dig in input {
            let next = dig.execute(loc);
            digs.push((loc, next));
            loc = next;
        }
        digs
    };

    let vertical_digs_by_column: BTreeMap<i64, BTreeSet<(i64, i64, ColDir, Turn, Turn)>> = digs
        .iter()
        .enumerate()
        .filter_map(|(i, (a, b))| {
            let digs = &digs;
            (a.1 == b.1).then(move || {
                let (col, (rowstart, rowend, coldir)) = if b.0 > a.0 {
                    (a.1, (a.0, b.0, ColDir::IncreasingAkaSouth))
                } else {
                    (a.1, (b.0, a.0, ColDir::DecreasingAkaNorth))
                };

                let prevdig = digs[(i - 1 + digs.len()) % digs.len()];
                assert_eq!(prevdig.1, *a);
                let prevloc = prevdig.0;

                let nextdig = digs[(i + 1 + digs.len()) % digs.len()];
                assert_eq!(*b, nextdig.0);
                let nextloc = nextdig.1;

                let prevturn = {
                    // Must have been horizontal.
                    assert_eq!(prevloc.0, a.0);
                    if prevloc.1 < a.1 {
                        // Previously was heading rightwards.
                        match coldir {
                            ColDir::IncreasingAkaSouth => Turn::Clockwise,
                            ColDir::DecreasingAkaNorth => Turn::CounterClockwise,
                        }
                    } else {
                        // previously was heading leftwards.
                        match coldir {
                            ColDir::IncreasingAkaSouth => Turn::CounterClockwise,
                            ColDir::DecreasingAkaNorth => Turn::Clockwise,
                        }
                    }
                };

                let nextturn = {
                    // Must next be horizontal.
                    assert_eq!(b.0, nextloc.0);
                    if b.1 < nextloc.1 {
                        // Next, heading rightwards.
                        match coldir {
                            ColDir::IncreasingAkaSouth => Turn::CounterClockwise,
                            ColDir::DecreasingAkaNorth => Turn::Clockwise,
                        }
                    } else {
                        // Next, heading leftwards.
                        match coldir {
                            ColDir::IncreasingAkaSouth => Turn::Clockwise,
                            ColDir::DecreasingAkaNorth => Turn::CounterClockwise,
                        }
                    }
                };
                (col, (rowstart, rowend, coldir, prevturn, nextturn))
            })
        })
        .into_grouping_map()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    let mut area = 0;
    let mut included_rows: BTreeSet<(i64, i64)> = BTreeSet::new();
    let mut prev_col = None;

    for (col, intervals) in vertical_digs_by_column {
        let prev_included_rows: BTreeSet<(i64, i64)> = included_rows.clone();
        let mut rows_to_remove: BTreeSet<(i64, i64)> = BTreeSet::new();
        for (lo, hi, dir, prevturn, nextturn) in intervals {
            match dir {
                ColDir::IncreasingAkaSouth => {
                    let keep_lo_included = match prevturn {
                        Turn::Clockwise => false,
                        Turn::CounterClockwise => true,
                    };
                    let keep_hi_included = match nextturn {
                        Turn::Clockwise => false,
                        Turn::CounterClockwise => true,
                    };
                    rows_to_remove.insert((
                        lo + if keep_lo_included { 1 } else { 0 },
                        hi - if keep_hi_included { 1 } else { 0 },
                    ));
                }
                ColDir::DecreasingAkaNorth => {
                    // Entering, so we can just include the edges.
                    included_rows.insert((lo, hi));
                }
            }
        }

        // Consolidate included_rows.
        included_rows = included_rows
            .into_iter()
            .coalesce(|prev, curr| {
                if prev.1 >= curr.0 - 1 {
                    Ok((prev.0.min(curr.0), prev.1.max(curr.1)))
                } else {
                    Err((prev, curr))
                }
            })
            .collect();

        // Remove any removed rows.
        for removed in rows_to_remove {
            let first_potential_overlap = included_rows.range(..(removed.0 - 1, 0)).rev().next();
            let last_potential_overlap = included_rows
                .range((Bound::Excluded((removed.1 + 1, 0)), Bound::Unbounded))
                .next();
            let all_overlaps = included_rows
                .range((
                    match first_potential_overlap {
                        None => Bound::Unbounded,
                        Some(first_potential_overlap) => Bound::Included(*first_potential_overlap),
                    },
                    match last_potential_overlap {
                        Some(x) => Bound::Included(*x),
                        None => Bound::Unbounded,
                    },
                ))
                .copied()
                .collect_vec();
            for segment in &all_overlaps {
                included_rows.remove(&segment);
                if segment.0 < removed.0 {
                    // There's stuff before.
                    included_rows.insert((segment.0, segment.1.min(removed.0 - 1)));
                }
                if segment.1 > removed.1 {
                    // There's stuff after.
                    included_rows.insert((segment.0.max(removed.1 + 1), segment.1));
                }
            }
            area += removed.1 - removed.0 + 1;
            // Reconsolidate.
            included_rows = included_rows
                .into_iter()
                .coalesce(|prev, curr| {
                    if prev.1 >= curr.0 - 1 {
                        Ok((prev.0.min(curr.0), prev.1.max(curr.1)))
                    } else {
                        Err((prev, curr))
                    }
                })
                .collect();
        }

        if let Some(prev_col) = prev_col {
            // Include all prev rows? New ones start now.
            for &row_range in &prev_included_rows {
                area += (col - prev_col) * (row_range.1 - row_range.0 + 1);
            }
        }

        prev_col = Some(col);
    }

    area
}

#[aoc(day18, part2)]
fn part2(input: &[Dig]) -> i64 {
    let input = input
        .into_iter()
        .map(
            |Dig {
                 dir: _,
                 dist: _,
                 color,
             }| {
                let dist = &color[..5];
                let dist = i64::from_str_radix(dist, 16).unwrap();

                let dir = &color[5..];
                let dir = match dir {
                    "0" => Dir::Right,
                    "1" => Dir::Down,
                    "2" => Dir::Left,
                    "3" => Dir::Up,
                    _ => panic!("{dir}"),
                };

                Dig {
                    dir,
                    dist,
                    color: String::new(),
                }
            },
        )
        .collect_vec();
    part1(&input)
}
