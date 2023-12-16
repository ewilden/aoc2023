use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{put_back_n, Itertools};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Spring {
    Fine,
    Damaged,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct InputLine {
    row: Vec<Option<Spring>>,
    sizes: Vec<usize>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Line {
    runs: Vec<Run>,
    sizes: Vec<usize>,
    force_next_damaged: bool,
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let (row, sizes) = line.split_once(" ").unwrap();
            let row = row.chars().map(|c| match c {
                '.' => Some(Spring::Fine),
                '#' => Some(Spring::Damaged),
                '?' => None,
                _ => panic!("{c}"),
            });
            let runs = row
                .group_by(|spring| *spring)
                .into_iter()
                .map(|(spring, springs)| {
                    let len = springs.count();
                    match spring {
                        Some(Spring::Damaged) => Run::Damaged(len),
                        Some(Spring::Fine) => Run::Fine(len),
                        None => Run::Unknown(len),
                    }
                })
                .collect_vec();
            let sizes = sizes
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect_vec();
            Line {
                runs,
                sizes,
                force_next_damaged: false,
            }
        })
        .collect_vec()
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Run {
    Fine(usize),
    Damaged(usize),
    Unknown(usize),
}

fn num_arrangements_inner(table: &mut HashMap<Line, usize>, line: Line) -> usize {
    let Line {
        runs,
        sizes,
        force_next_damaged,
    } = line;

    let mut runs = put_back_n(runs.iter().copied());
    let mut sizes = put_back_n(sizes.iter().copied());

    let size = sizes.next();
    let run = runs.next();

    fn consume_one_fine_and_recurse(
        table: &mut HashMap<Line, usize>,
        sizes: itertools::PutBackN<std::iter::Copied<std::slice::Iter<'_, usize>>>,
        mut runs: itertools::PutBackN<std::iter::Copied<std::slice::Iter<'_, Run>>>,
    ) -> usize {
        // Now we need at least one "fine" in order to recurse, or to have reached the end.
        match runs.next() {
            None => {
                if sizes.count() == 0 {
                    1
                } else {
                    0
                }
            }
            Some(run) => {
                match run {
                    Run::Damaged(_) => 0,
                    Run::Fine(next_fine) => {
                        if next_fine == 1 {
                            // Ok, we consume this and recurse.
                            num_arrangements(
                                table,
                                Line {
                                    sizes: sizes.collect_vec(),
                                    runs: runs.collect_vec(),
                                    force_next_damaged: false,
                                },
                            )
                        } else {
                            // Take away one (must still be nonzero), then put back and recurse.
                            runs.put_back(Run::Fine(next_fine.checked_sub(1).unwrap()));
                            num_arrangements(
                                table,
                                Line {
                                    sizes: sizes.collect_vec(),
                                    runs: runs.collect_vec(),
                                    force_next_damaged: false,
                                },
                            )
                        }
                    }
                    Run::Unknown(next_unknown) => {
                        if next_unknown == 1 {
                            // Ok, we consume this and recurse.
                            num_arrangements(
                                table,
                                Line {
                                    sizes: sizes.collect_vec(),
                                    runs: runs.collect_vec(),
                                    force_next_damaged: false,
                                },
                            )
                        } else {
                            // Take away one (must still be nonzero), then put back and recurse.
                            runs.put_back(Run::Unknown(next_unknown.checked_sub(1).unwrap()));
                            num_arrangements(
                                table,
                                Line {
                                    sizes: sizes.collect_vec(),
                                    runs: runs.collect_vec(),
                                    force_next_damaged: false,
                                },
                            )
                        }
                    }
                }
            }
        }
    }

    match (size, run) {
        (None, None) => 1,
        (None, Some(run)) => {
            runs.put_back(run);
            if runs.any(|run| matches!(run, Run::Damaged(_))) {
                0
            } else {
                1
            }
        }
        (Some(_size), None) => 0,
        (Some(size), Some(run)) => match run {
            Run::Fine(_) => {
                if force_next_damaged {
                    0
                } else {
                    sizes.put_back(size);
                    num_arrangements(
                        table,
                        Line {
                            runs: runs.collect_vec(),
                            sizes: sizes.collect_vec(),
                            force_next_damaged: false,
                        },
                    )
                }
            }
            Run::Damaged(this_damaged) => match size.cmp(&this_damaged) {
                std::cmp::Ordering::Less => 0,
                std::cmp::Ordering::Equal => consume_one_fine_and_recurse(table, sizes, runs),
                std::cmp::Ordering::Greater => {
                    sizes.put_back(size.checked_sub(this_damaged).unwrap());
                    num_arrangements(
                        table,
                        Line {
                            runs: runs.collect_vec(),
                            sizes: sizes.collect_vec(),
                            force_next_damaged: true,
                        },
                    )
                }
            },
            Run::Unknown(this_unknown) => {
                // Recurse treating this as 1 damaged or 1 not damaged.
                let after_1 = this_unknown - 1;
                sizes.put_back(size);
                let sizes = sizes.collect_vec();
                if after_1 > 0 {
                    runs.put_back(Run::Unknown(after_1));
                }

                let runs = runs.collect_vec();

                // as 1 damaged
                let mut runs_with_one_damaged = put_back_n(runs.iter().copied());
                runs_with_one_damaged.put_back(Run::Damaged(1));

                let ways_with_one_damaged = num_arrangements(
                    table,
                    Line {
                        runs: runs_with_one_damaged.collect_vec(),
                        sizes: sizes.clone(),
                        force_next_damaged,
                    },
                );

                // as 1 fine
                let mut runs_with_one_fine = put_back_n(runs.iter().copied());
                runs_with_one_fine.put_back(Run::Fine(1));

                let ways_with_one_fine = num_arrangements(
                    table,
                    Line {
                        runs: runs_with_one_fine.collect_vec(),
                        sizes: sizes.clone(),
                        force_next_damaged,
                    },
                );

                ways_with_one_damaged + ways_with_one_fine
            }
        },
    }
}

fn num_arrangements(table: &mut HashMap<Line, usize>, line: Line) -> usize {
    if let Some(n) = table.get(&line) {
        return *n;
    }
    let n = num_arrangements_inner(table, line.clone());
    table.insert(line.clone(), n);
    n
}

#[aoc(day12, part1)]
fn part1(input: &[Line]) -> usize {
    let mut table = HashMap::new();
    input
        .iter()
        .map(|line| num_arrangements(&mut table, line.clone()))
        .sum()
}

#[aoc(day12, part2)]
fn part2(input: &[Line]) -> usize {
    let mut table = HashMap::new();
    input
        .iter()
        .map(|line| {
            let Line {
                runs,
                sizes,
                force_next_damaged: _,
            } = line.clone();
            let runs = (1..=5)
                .map(|_| runs.clone())
                .into_iter()
                .intersperse(vec![Run::Unknown(1)])
                .flatten()
                .collect_vec();
            let sizes = (1..=5).map(|_| sizes.clone()).flatten().collect_vec();
            num_arrangements(
                &mut table,
                Line {
                    runs,
                    sizes,
                    force_next_damaged: false,
                },
            )
        })
        .sum()
}
