use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use petgraph::unionfind::UnionFind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Dig {
    dir: Dir,
    dist: i32,
    color: String,
}

impl Dir {
    fn apply(&self, (row, col): (i32, i32)) -> (i32, i32) {
        match self {
            Dir::Up => (row - 1, col),
            Dir::Right => (row, col + 1),
            Dir::Down => (row + 1, col),
            Dir::Left => (row, col - 1),
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
            let dist = dist.parse::<i32>().unwrap();
            let color = color.strip_prefix("(#").unwrap().strip_suffix(')').unwrap();
            Dig {
                dir,
                dist,
                color: color.to_owned(),
            }
        })
        .collect_vec()
}

#[aoc(day18, part1)]
fn part1(input: &[Dig]) -> usize {
    let mut curr = (0i32, 0i32);
    let mut dug = HashSet::from([curr]);
    for Dig {
        dir,
        dist,
        color: _,
    } in input
    {
        for _ in 0..*dist {
            curr = dir.apply(curr);
            dug.insert(curr);
        }
    }

    let (min_row, max_row) = dug.iter().map(|x| x.0).minmax().into_option().unwrap();
    let (min_col, max_col) = dug.iter().map(|x| x.1).minmax().into_option().unwrap();

    let num_spots_considered =
        usize::try_from(((max_row + 2) - (min_row - 2) + 1) * ((max_col + 2) - (min_col - 2) + 1))
            .unwrap();

    let to_key = |(r, c)| {
        let r = r - (min_row - 2);
        let c = c - (min_col - 2);
        usize::try_from(r * ((max_col + 2) - (min_col - 2) + 1) + c).unwrap()
    };

    // Using usize is a dumb hack here, petgraph should really support u64 as an IndexType.
    let mut finder = UnionFind::<usize>::new(num_spots_considered * 2);

    for r in min_row - 1..=max_row + 1 {
        for c in min_col - 1..=max_col + 1 {
            let loc = (r, c);
            for neighbor in (r - 1..=r + 1).cartesian_product(c - 1..=c + 1) {
                if dug.contains(&loc) == dug.contains(&neighbor) {
                    finder.union(to_key(loc), to_key(neighbor));
                }
            }
        }
    }

    let outside_label = finder.find(to_key((min_row - 1, min_col - 1)));
    finder
        .into_labeling()
        .into_iter()
        .filter(|label| *label != outside_label)
        .counts()
        .into_iter()
        .sorted_by_key(|x| x.1)
        .rev()
        .take(2)
        .map(|x| x.1)
        .sum()
}

#[aoc(day18, part2)]
fn part2(input: &[Dig]) -> usize {
    let input = input
        .into_iter()
        .map(
            |Dig {
                 dir: _,
                 dist: _,
                 color,
             }| {
                let dist = &color[..5];
                let dist = i32::from_str_radix(dist, 16).unwrap();

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
