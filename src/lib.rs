mod day8;
mod day7;
mod day6;
mod day5;
mod day4;
mod day3;
mod day2;
mod day1;
use aoc_runner_derive::aoc_lib;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

aoc_lib! { year = 2023 }
