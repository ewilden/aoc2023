use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<Vec<Vec<bool>>> {
    input
        .trim()
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => false,
                            '#' => true,
                            _ => panic!("{c}"),
                        })
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec()
}

fn xor(
    a: impl IntoIterator<Item = bool>,
    b: impl IntoIterator<Item = bool>,
) -> impl Iterator<Item = bool> {
    let a = a.into_iter();
    let b = b.into_iter();
    a.zip_eq(b).map(|(a, b)| a ^ b)
}

fn transpose(grid: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut transposed = Vec::new();
    for c in 0..grid[0].len() {
        let mut row = Vec::new();
        for r in 0..grid.len() {
            row.push(grid[r][c]);
        }
        transposed.push(row);
    }
    transposed
}

#[derive(Debug)]
struct RowCursor<'a> {
    grid: &'a [Vec<bool>],
    sum: Vec<bool>,
    curr: usize,
}

impl<'a> RowCursor<'a> {
    fn next(self) -> Self {
        let RowCursor { grid, sum, curr } = self;

        let halfwidth = curr.min(grid.len() - curr);
        let current_first = curr - halfwidth;
        let current_last = curr + halfwidth - 1;

        let next = curr + 1;
        let next_halfwidth = next.min(grid.len() - next);
        let next_first = next - next_halfwidth;
        let next_last = next + next_halfwidth - 1;

        let mut sum = sum;
        for i in current_first..next_first {
            sum = xor(sum, grid[i].iter().copied()).collect_vec();
        }

        for i in (current_last + 1)..=next_last {
            sum = xor(sum, grid[i].iter().copied()).collect_vec();
        }

        RowCursor {
            grid,
            sum,
            curr: curr + 1,
        }
    }

    fn check_mirror(&self) -> bool {
        let RowCursor { grid, sum, curr } = self;
        let curr = *curr;

        if sum.iter().any(|x| *x) {
            return false;
        }

        let halfwidth = curr.min(grid.len() - curr);
        let current_first = curr - halfwidth;
        let current_last = curr + halfwidth - 1;

        grid[current_first..=current_last]
            .iter()
            .rev()
            .zip_eq(grid[current_first..=current_last].iter())
            .all(|(a, b)| a == b)
    }

    fn check_smudge(&self) -> bool {
        let RowCursor { grid, sum, curr } = self;
        if sum.iter().filter(|x| **x).count() != 1 {
            return false;
        }

        let curr = *curr;
        let halfwidth = curr.min(grid.len() - curr);
        let current_first = curr - halfwidth;
        let current_last = curr + halfwidth - 1;

        grid[current_first..=current_last]
            .iter()
            .rev()
            .zip_eq(grid[current_first..=current_last].iter())
            .flat_map(|(a, b)| a.iter().copied().zip_eq(b.iter().copied()))
            .filter(|(a, b)| a != b)
            .count()
            == 2 // (it's 2 because we're double-counting.)
    }
}

fn find_row_mirror(grid: &[Vec<bool>]) -> Option<usize> {
    let mut cursor = RowCursor {
        grid,
        sum: xor(grid[0].iter().copied(), grid[1].iter().copied()).collect_vec(),
        curr: 1,
    };

    while cursor.curr < grid.len() - 1 {
        if cursor.check_mirror() {
            return Some(cursor.curr);
        }
        cursor = cursor.next();
    }

    if cursor.check_mirror() {
        return Some(cursor.curr);
    }

    None
}

fn find_row_smudge(grid: &[Vec<bool>]) -> Option<usize> {
    let mut cursor = RowCursor {
        grid,
        sum: xor(grid[0].iter().copied(), grid[1].iter().copied()).collect_vec(),
        curr: 1,
    };

    while cursor.curr < grid.len() - 1 {
        if cursor.check_smudge() {
            return Some(cursor.curr);
        }
        cursor = cursor.next();
    }

    if cursor.check_smudge() {
        return Some(cursor.curr);
    }

    None
}

#[aoc(day13, part1)]
fn part1(input: &[Vec<Vec<bool>>]) -> usize {
    input
        .into_iter()
        .map(|grid| {
            find_row_mirror(grid)
                .map(|x| x * 100)
                .or(find_row_mirror(&transpose(grid)))
                .expect("never found winning row")
        })
        .sum()
}

#[aoc(day13, part2)]
fn part2(input: &[Vec<Vec<bool>>]) -> usize {
    input
        .into_iter()
        .map(|grid| {
            find_row_smudge(grid)
                .map(|x| x * 100)
                .or(find_row_smudge(&transpose(grid)))
                .expect("never found winning row")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1sample() {
        eprintln!(
            "{}",
            super::part1(&super::parse(
                "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
"
            ))
        );
    }
}
