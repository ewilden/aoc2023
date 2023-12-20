use std::{
    collections::{HashMap, HashSet},
    iter::repeat,
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn apply(&self, (row, col): (i64, i64)) -> (i64, i64) {
        match self {
            Dir::Up => (row - 1, col),
            Dir::Right => (row, col + 1),
            Dir::Down => (row + 1, col),
            Dir::Left => (row, col - 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Device {
    Mirror(Mirror),
    Splitter(Splitter),
}

impl Device {
    fn apply(&self, dir: Dir) -> Vec<Dir> {
        match self {
            Device::Mirror(mirror) => vec![mirror.apply(dir)],
            Device::Splitter(splitter) => splitter.apply(dir),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Mirror {
    FwdSlash,
    BackSlash,
}

impl Mirror {
    fn apply(&self, dir: Dir) -> Dir {
        match (self, dir) {
            (Mirror::FwdSlash, Dir::Up) => Dir::Right,
            (Mirror::FwdSlash, Dir::Right) => Dir::Up,
            (Mirror::FwdSlash, Dir::Down) => Dir::Left,
            (Mirror::FwdSlash, Dir::Left) => Dir::Down,
            (Mirror::BackSlash, Dir::Up) => Dir::Left,
            (Mirror::BackSlash, Dir::Right) => Dir::Down,
            (Mirror::BackSlash, Dir::Down) => Dir::Right,
            (Mirror::BackSlash, Dir::Left) => Dir::Up,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Splitter {
    Pipe,
    Dash,
}

impl Splitter {
    fn apply(&self, dir: Dir) -> Vec<Dir> {
        match (self, dir) {
            (Splitter::Pipe, Dir::Up) => vec![Dir::Up],
            (Splitter::Pipe, Dir::Right) => vec![Dir::Up, Dir::Down],
            (Splitter::Pipe, Dir::Down) => vec![Dir::Down],
            (Splitter::Pipe, Dir::Left) => vec![Dir::Up, Dir::Down],
            (Splitter::Dash, Dir::Up) => vec![Dir::Left, Dir::Right],
            (Splitter::Dash, Dir::Right) => vec![Dir::Right],
            (Splitter::Dash, Dir::Down) => vec![Dir::Left, Dir::Right],
            (Splitter::Dash, Dir::Left) => vec![Dir::Left],
        }
    }
}

#[aoc_generator(day16)]
fn parse(input: &str) -> Vec<Vec<Option<Device>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => None,
                    c => Some(match c {
                        '/' => Device::Mirror(Mirror::FwdSlash),
                        '\\' => Device::Mirror(Mirror::BackSlash),
                        '|' => Device::Splitter(Splitter::Pipe),
                        '-' => Device::Splitter(Splitter::Dash),
                        _ => panic!("{c}"),
                    }),
                })
                .collect_vec()
        })
        .collect_vec()
}

#[aoc(day16, part1)]
fn part1(input: &[Vec<Option<Device>>]) -> usize {
    let grid: HashMap<(i64, i64), Device> = input
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(c, dev)| match dev {
                    Some(dev) => {
                        Some(((i64::try_from(r).unwrap(), i64::try_from(c).unwrap()), *dev))
                    }
                    None => None,
                })
        })
        .collect::<HashMap<_, _>>();

    let num_rows = i64::try_from(input.len()).unwrap();
    let num_cols = i64::try_from(input[0].len()).unwrap();
    let boundcheck = |(r, c)| (0..num_rows).contains(&r) && (0..num_cols).contains(&c);

    let mut curr_beams = vec![((0, 0), Dir::Right)];
    let mut seen_beams: HashSet<((i64, i64), Dir)> = curr_beams.iter().copied().collect();

    {
        let grid = &grid;
        let seen_beams = &mut seen_beams;

        while !curr_beams.is_empty() {
            let beams = std::mem::take(&mut curr_beams);
            for (coords, dir) in beams {
                let dirs = grid
                    .get(&coords)
                    .map(|dev| dev.apply(dir))
                    .unwrap_or_else(|| vec![dir]);
                for dir in dirs {
                    let coords = dir.apply(coords);
                    if boundcheck(coords) && seen_beams.insert((coords, dir)) {
                        curr_beams.push((coords, dir));
                    }
                }
            }
        }
    }

    seen_beams
        .into_iter()
        .map(|(coords, _)| coords)
        .unique()
        .count()
}

#[aoc(day16, part2)]
fn part2(input: &[Vec<Option<Device>>]) -> usize {
    let grid: HashMap<(i64, i64), Device> = input
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(c, dev)| match dev {
                    Some(dev) => {
                        Some(((i64::try_from(r).unwrap(), i64::try_from(c).unwrap()), *dev))
                    }
                    None => None,
                })
        })
        .collect::<HashMap<_, _>>();

    let num_rows = i64::try_from(input.len()).unwrap();
    let num_cols = i64::try_from(input[0].len()).unwrap();
    let boundcheck = |(r, c)| (0..num_rows).contains(&r) && (0..num_cols).contains(&c);

    let mut best_start = 0usize;

    for start in (0..num_rows)
        .zip(repeat(0i64))
        .map(|x| (x, Dir::Right))
        .chain(
            (0..num_rows)
                .zip(repeat(num_cols - 1))
                .map(|x| (x, Dir::Left)),
        )
        .chain(repeat(0i64).zip(0..num_cols).map(|x| (x, Dir::Down)))
        .chain(repeat(num_rows - 1).zip(0..num_cols).map(|x| (x, Dir::Up)))
    {
        let mut curr_beams = vec![start];
        let mut seen_beams: HashSet<((i64, i64), Dir)> = curr_beams.iter().copied().collect();

        {
            let grid = &grid;
            let seen_beams = &mut seen_beams;

            while !curr_beams.is_empty() {
                let beams = std::mem::take(&mut curr_beams);
                for (coords, dir) in beams {
                    let dirs = grid
                        .get(&coords)
                        .map(|dev| dev.apply(dir))
                        .unwrap_or_else(|| vec![dir]);
                    for dir in dirs {
                        let coords = dir.apply(coords);
                        if boundcheck(coords) && seen_beams.insert((coords, dir)) {
                            curr_beams.push((coords, dir));
                        }
                    }
                }
            }
        }

        best_start = best_start.max(
            seen_beams
                .into_iter()
                .map(|(coords, _)| coords)
                .unique()
                .count(),
        )
    }
    best_start
}
