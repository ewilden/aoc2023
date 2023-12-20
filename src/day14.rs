use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Rock {
    Round,
    Cube,
}

#[aoc_generator(day14)]
fn parse(input: &str) -> (HashMap<(usize, usize), Rock>, usize, usize) {
    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().chars().count();
    (
        input
            .lines()
            .enumerate()
            .flat_map(|(r, line)| {
                line.char_indices().flat_map(move |(c, rock)| {
                    let rock = match rock {
                        '#' => Some(Rock::Cube),
                        'O' => Some(Rock::Round),
                        '.' => None,
                        _ => panic!("{rock}"),
                    };
                    rock.map(|rock| ((r, c), rock))
                })
            })
            .collect(),
        num_rows,
        num_cols,
    )
}

#[aoc(day14, part1)]
fn part1(input: &(HashMap<(usize, usize), Rock>, usize, usize)) -> usize {
    let (input, num_rows, num_cols) = input;
    let mut sum = 0;
    for c in 0..*num_cols {
        let mut next_empty = 0;
        for r in 0..*num_rows {
            match input.get(&(r, c)) {
                Some(rock) => match rock {
                    Rock::Round => {
                        sum += *num_rows - next_empty;
                        next_empty += 1;
                    }
                    Rock::Cube => {
                        next_empty = r + 1;
                    }
                },
                None => {}
            }
        }
    }
    sum
}

fn cycle(grid: &mut HashMap<(usize, usize), Rock>, num_rows: usize, num_cols: usize) {
    // north
    for c in 0..num_cols {
        let mut next_empty = 0;
        for r in 0..num_rows {
            match grid.remove(&(r, c)) {
                Some(rock) => match rock {
                    Rock::Round => {
                        assert!(grid.insert((next_empty, c), Rock::Round).is_none());
                        next_empty += 1;
                    }
                    Rock::Cube => {
                        assert!(grid.insert((r, c), Rock::Cube).is_none());
                        next_empty = r + 1;
                    }
                },
                None => {}
            }
        }
    }

    // west
    for r in 0..num_rows {
        let mut next_empty = 0;
        for c in 0..num_cols {
            match grid.remove(&(r, c)) {
                Some(rock) => match rock {
                    Rock::Round => {
                        assert!(grid.insert((r, next_empty), Rock::Round).is_none());
                        next_empty += 1;
                    }
                    Rock::Cube => {
                        assert!(grid.insert((r, c), Rock::Cube).is_none());
                        next_empty = c + 1;
                    }
                },
                None => {}
            }
        }
    }

    // south
    for c in 0..num_cols {
        let mut next_empty = num_rows - 1;
        for r in (0..num_rows).rev() {
            match grid.remove(&(r, c)) {
                Some(rock) => match rock {
                    Rock::Round => {
                        assert!(grid.insert((next_empty, c), Rock::Round).is_none());
                        next_empty -= 1;
                    }
                    Rock::Cube => {
                        assert!(grid.insert((r, c), Rock::Cube).is_none());
                        next_empty = r - 1;
                    }
                },
                None => {}
            }
        }
    }

    // east
    for r in 0..num_rows {
        let mut next_empty = num_cols - 1;
        for c in (0..num_cols).rev() {
            match grid.remove(&(r, c)) {
                Some(rock) => match rock {
                    Rock::Round => {
                        assert!(grid.insert((r, next_empty), Rock::Round).is_none());
                        next_empty -= 1;
                    }
                    Rock::Cube => {
                        assert!(grid.insert((r, c), Rock::Cube).is_none());
                        next_empty = c - 1;
                    }
                },
                None => {}
            }
        }
    }
}

#[aoc(day14, part2)]
fn part2(input: &(HashMap<(usize, usize), Rock>, usize, usize)) -> usize {
    let (input, num_rows, num_cols) = input;
    let num_rows = *num_rows;
    let num_cols = *num_cols;

    let mut tortoise = input.clone();
    let mut hare = input.clone();

    let mut period_start = None;
    let mut now = 0usize;
    let period_end = loop {
        if tortoise == hare {
            match period_start {
                Some(_) => break now,
                None => {
                    assert!(period_start.replace(now).is_none());
                }
            }
        }
        now += 1;
        cycle(&mut tortoise, num_rows, num_cols);
        cycle(&mut hare, num_rows, num_cols);
        cycle(&mut hare, num_rows, num_cols);
    };
    let period_start = period_start.unwrap();
    let period = period_end - period_start;
    assert!(period_end < 1000000000);
    let remaining_cycles = (1000000000 - period_end) % period;

    for _ in 0..remaining_cycles {
        cycle(&mut tortoise, num_rows, num_cols);
    }

    tortoise
        .into_iter()
        .filter_map(|((r, _), rock)| match rock {
            Rock::Round => Some(num_rows - r),
            Rock::Cube => None,
        })
        .sum()
}
