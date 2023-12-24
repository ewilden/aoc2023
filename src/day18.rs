use std::{collections::BTreeMap, ops::Range};

use aoc_runner_derive::{aoc, aoc_generator};
use intervaltree::IntervalTree;
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
struct ColSpec {
    col: i64,
    hits_start: bool,
    hits_middle: bool,
    hits_end: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum InsideState {
    Outside,
    Inside,
    EnteredTop,
    ExitedTop,
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

    for (a, b) in input.iter().tuple_windows::<(_, _)>() {
        let dirs = [a.dir, b.dir];
        // No double backs or continues?
        assert_eq!(
            dirs.contains(&Dir::Up) || dirs.contains(&Dir::Down),
            dirs.contains(&Dir::Right) || dirs.contains(&Dir::Left)
        );
        // No length-1 digs?
        assert!(a.dist > 1);
        assert!(b.dist > 1);
    }

    let gather_rows = |digs: &[((i64, i64), (i64, i64))]| {
        let row_to_digs_in_that_row = digs
            .iter()
            .filter_map(|dig| {
                let (a, b) = *dig;
                (a.0 == b.0).then_some((a.0, a.1.min(b.1)..a.1.max(b.1) + 1))
            })
            .into_grouping_map_by(|entry| {
                let (row, _col_range) = entry;
                *row
            })
            .collect::<Vec<(i64, Range<i64>)>>();
        row_to_digs_in_that_row
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    IntervalTree::from_iter(v.into_iter().map(|(r, range)| {
                        assert_eq!(k, r);
                        (range, ())
                    })),
                )
            })
            .collect::<BTreeMap<_, _>>()
    };

    let row_to_digs_in_that_row = gather_rows(&digs);
    let col_to_digs_in_that_col = gather_rows(
        &digs
            .iter()
            .map(|(a, b)| ((a.1, a.0), (b.1, b.0)))
            .collect_vec(),
    );

    let mut area = 0i64;
    for (position, (rowstart, rowend)) in row_to_digs_in_that_row
        .keys()
        .copied()
        .tuple_windows::<(_, _)>()
        .with_position()
    {
        let is_last_row = match position {
            itertools::Position::Last => true,
            itertools::Position::Only => unreachable!(),
            _ => false,
        };
        let height = rowend - rowstart;
        let mut inside = InsideState::Outside;

        for (position, (first_col, last_col)) in col_to_digs_in_that_col
            .iter()
            .filter_map(|(col, col_digs)| {
                let hits_start = col_digs.query_point(rowstart).next().is_some();
                let hits_middle = col_digs.query_point(rowstart + 1).next().is_some();
                let hits_end = col_digs.query_point(rowend).next().is_some();
                (hits_start).then_some(ColSpec {
                    col: *col,
                    hits_start,
                    hits_middle,
                    hits_end,
                })
            })
            .tuple_windows::<(_, _)>()
            .with_position()
        {
            let is_last_col = match position {
                itertools::Position::First | itertools::Position::Middle => false,
                itertools::Position::Last | itertools::Position::Only => true,
            };

            inside = match (inside, first_col.hits_middle) {
                (InsideState::Outside, true) => InsideState::Inside,
                (InsideState::Outside, false) => InsideState::EnteredTop,
                (InsideState::Inside, true) => InsideState::Outside,
                (InsideState::Inside, false) => InsideState::ExitedTop,
                (InsideState::EnteredTop, true) => InsideState::Inside,
                (InsideState::EnteredTop, false) => InsideState::Outside,
                (InsideState::ExitedTop, true) => InsideState::Outside,
                (InsideState::ExitedTop, false) => InsideState::Inside,
            };

            let width = last_col.col - first_col.col;

            assert!(width > 1);
            let to_add_bottom_length = if row_to_digs_in_that_row
                .get(&rowend)
                .unwrap()
                .query_point(first_col.col + 1)
                .next()
                .is_some()
            {
                // Include the bottom of this row as well.
                width
            } else {
                0
            };

            match inside {
                InsideState::Outside => {}
                InsideState::Inside => {
                    area += height * width + to_add_bottom_length;
                }
                InsideState::EnteredTop => {
                    // area += width;
                }
                InsideState::ExitedTop => {
                    area += height * width + to_add_bottom_length;
                }
            }

            // if matches!(inside, InsideState::Outside) && first_col.hits_end {
            //     area += height;
            // }

            // match (inside_top, inside_bottom) {
            //     (true, true) => {
            //         area += height * width;
            //     }
            //     (true, false) => {
            //         area += width;
            //     }
            //     (false, true) => {
            //         area += width;
            //     }
            //     (false, false) => {
            //         // Not inside.
            //         // if inside_
            //     }
            // }
        }

        // assert!(
        //     matches!(
        //         inside,
        //         InsideState::Inside | InsideState::ExitedTop | InsideState::EnteredTop
        //     ),
        //     "not inside at end of {rowstart}..={rowend}"
        // );
        // area += height;
        // assert!(inside_top && inside_bottom);
        // assert!(saw_anything);
        // if is_last_row {
        //     for dig in row_to_digs_in_that_row.get(&rowend).unwrap().iter_sorted() {
        //         let Range { start, end } = dig.range;
        //         area += end - start + 1;
        //     }
        // }
    }

    // for elem in row_to_digs_in_that_row
    //     .first_key_value()
    //     .unwrap()
    //     .1
    //     .iter_sorted()
    // {
    //     let Range { start, end } = elem.range;
    //     area += end - start;
    // }

    // for dig in input {
    //     area += dig.dist;
    // }
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
