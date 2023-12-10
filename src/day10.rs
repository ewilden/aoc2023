use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{Itertools, MinMaxResult};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Dir {
    N,
    E,
    S,
    W,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Tile {
    Ground,
    Start,
    Pipe([Dir; 2]),
}

type Coords = (i64, i64);

#[aoc_generator(day10)]
fn parse(input: &str) -> HashMap<Coords, Tile> {
    let pipe = |dirs| Tile::Pipe(dirs);
    use Dir::*;

    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '|' => pipe([N, S]),
                    '-' => pipe([E, W]),
                    'L' => pipe([N, E]),
                    'J' => pipe([N, W]),
                    '7' => pipe([S, W]),
                    'F' => pipe([S, E]),
                    '.' => Tile::Ground,
                    'S' => Tile::Start,
                    _ => panic!("{c}"),
                })
                .enumerate()
        })
        .enumerate()
        .flat_map(|(r, iter)| {
            iter.map(move |(c, tile)| {
                (
                    (i64::try_from(r + 1).unwrap(), i64::try_from(c + 1).unwrap()),
                    tile,
                )
            })
        })
        .collect()
}

fn apply((r, c): Coords, dir: Dir) -> Coords {
    match dir {
        Dir::N => (r - 1, c),
        Dir::E => (r, c + 1),
        Dir::S => (r + 1, c),
        Dir::W => (r, c - 1),
    }
}

fn neighborhood(input: &HashMap<Coords, Tile>, loc: Coords) -> Vec<Coords> {
    let tile = *input.get(&loc).unwrap();
    match tile {
        Tile::Ground => vec![],
        Tile::Start => [
            apply(loc, Dir::N),
            apply(loc, Dir::E),
            apply(loc, Dir::S),
            apply(loc, Dir::W),
        ]
        .into_iter()
        .filter(|next_loc| {
            input.contains_key(next_loc) && neighborhood(input, *next_loc).contains(&loc)
        })
        .collect(),
        Tile::Pipe([a, b]) => [apply(loc, a), apply(loc, b)]
            .into_iter()
            .filter(|loc| input.contains_key(loc))
            .collect(),
    }
}

#[aoc(day10, part1)]
fn part1(input: &HashMap<Coords, Tile>) -> i32 {
    let neighborhood = |loc| neighborhood(input, loc);

    let start_loc = input
        .iter()
        .find_map(|(coords, tile)| (*tile == Tile::Start).then_some(*coords))
        .unwrap();

    let mut visited = HashMap::from([(start_loc, 0)]);
    let mut recent = HashSet::from([start_loc]);

    let mut step = 1;
    loop {
        let nexts = recent
            .iter()
            .copied()
            .flat_map(&neighborhood)
            .filter(|loc| !visited.contains_key(loc))
            .collect::<HashSet<_>>();
        if nexts.is_empty() {
            break;
        }
        visited.extend(nexts.iter().copied().map(|loc| (loc, step)));
        recent = nexts;
        step += 1;
    }

    visited.into_values().max().unwrap()
}

#[aoc(day10, part2)]
fn part2(input: &HashMap<Coords, Tile>) -> usize {
    let neighborhood = |loc| neighborhood(input, loc);

    let start_loc = input
        .iter()
        .find_map(|(coords, tile)| (*tile == Tile::Start).then_some(*coords))
        .unwrap();

    let mut visited = HashMap::from([(start_loc, 0)]);
    let mut recent = HashSet::from([start_loc]);

    let mut step = 1;
    loop {
        let nexts = recent
            .iter()
            .copied()
            .flat_map(&neighborhood)
            .filter(|loc| !visited.contains_key(loc))
            .collect::<HashSet<_>>();
        if nexts.is_empty() {
            break;
        }
        visited.extend(nexts.iter().copied().map(|loc| (loc, step)));
        recent = nexts;
        step += 1;
    }

    let pipes = visited.into_keys().collect::<HashSet<_>>();
    let MinMaxResult::MinMax(min_r, max_r) = input.keys().map(|(r, _)| *r).minmax() else {
        unreachable!()
    };
    let MinMaxResult::MinMax(min_c, max_c) = input.keys().map(|(_, c)| *c).minmax() else {
        unreachable!()
    };

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    enum EntryState {
        Outside,
        Inside,
        EnteringN,
        EnteringS,
        ExitingN,
        ExitingS,
    }

    let mut entry_state = EntryState::Outside;
    let mut inside_count = 0usize;

    let get_pipe = |loc| {
        let tile = *input.get(&loc).unwrap();
        match tile {
            Tile::Pipe(pipe) => pipe,
            Tile::Start => {
                let nbrs = neighborhood(loc);
                [Dir::N, Dir::S, Dir::E, Dir::W]
                    .into_iter()
                    .filter(|dir| nbrs.contains(&apply(loc, *dir)))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            }
            _ => panic!("{tile:?}"),
        }
    };

    for r in min_r - 1..=max_r + 1 {
        assert_eq!(
            entry_state,
            EntryState::Outside,
            "{r} {:?}",
            pipes
                .iter()
                .filter(|(row, _)| *row == r)
                .sorted()
                .collect_vec()
        );
        for c in min_c - 1..=max_c + 1 {
            if pipes.contains(&(r, c)) {
                let pipe = get_pipe((r, c));
                match entry_state {
                    EntryState::Outside => {
                        if pipe.contains(&Dir::N) && pipe.contains(&Dir::S) {
                            entry_state = EntryState::Inside;
                        } else if pipe.contains(&Dir::N) {
                            entry_state = EntryState::EnteringN;
                        } else if pipe.contains(&Dir::S) {
                            entry_state = EntryState::EnteringS;
                        }
                    }
                    EntryState::Inside => {
                        if pipe.contains(&Dir::N) && pipe.contains(&Dir::S) {
                            entry_state = EntryState::Outside;
                        } else if pipe.contains(&Dir::N) {
                            entry_state = EntryState::ExitingN;
                        } else if pipe.contains(&Dir::S) {
                            entry_state = EntryState::ExitingS;
                        }
                    }
                    EntryState::EnteringN => {
                        if pipe.contains(&Dir::N) {
                            entry_state = EntryState::Outside;
                        } else if pipe.contains(&Dir::S) {
                            entry_state = EntryState::Inside;
                        }
                    }
                    EntryState::EnteringS => {
                        if pipe.contains(&Dir::N) {
                            entry_state = EntryState::Inside;
                        } else if pipe.contains(&Dir::S) {
                            entry_state = EntryState::Outside;
                        }
                    }
                    EntryState::ExitingN => {
                        if pipe.contains(&Dir::N) {
                            entry_state = EntryState::Inside;
                        } else if pipe.contains(&Dir::S) {
                            entry_state = EntryState::Outside;
                        }
                    }
                    EntryState::ExitingS => {
                        if pipe.contains(&Dir::N) {
                            entry_state = EntryState::Outside;
                        } else if pipe.contains(&Dir::S) {
                            entry_state = EntryState::Inside;
                        }
                    }
                }
            } else {
                if entry_state == EntryState::Inside {
                    inside_count += 1;
                }
            }
        }
    }

    inside_count
}
