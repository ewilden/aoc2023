use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

fn hash(s: impl IntoIterator<Item = char>) -> u8 {
    s.into_iter().fold(0u8, |curr, c| {
        let ascii_code = u8::try_from(c).unwrap();
        let curr = u32::from(curr) + u32::from(ascii_code);
        let curr = curr * 17;
        u8::try_from(curr % 256).unwrap()
    })
}

#[aoc_generator(day15, part1)]
fn parse(input: &str) -> Vec<String> {
    input
        .trim()
        .replace('\n', "")
        .split(',')
        .map(|x| x.to_owned())
        .collect_vec()
}

#[aoc(day15, part1)]
fn part1(input: &[String]) -> u64 {
    input
        .iter()
        .map(|instr| hash(instr.chars()))
        .map(u64::from)
        .sum()
}

type FocalLength = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Op {
    Dash,
    Equals(FocalLength),
}

#[aoc_generator(day15, part2)]
fn parse2(input: &str) -> Vec<(String, Op)> {
    input
        .trim()
        .replace('\n', "")
        .split(',')
        .map(|x| {
            let (label, op) = x.split_once(|c| ['=', '-'].contains(&c)).unwrap();
            let op = if x.contains('=') {
                Op::Equals(op.parse::<FocalLength>().unwrap())
            } else {
                assert_eq!(op, "");
                assert!(x.contains('-'));
                Op::Dash
            };
            (label.to_owned(), op)
        })
        .collect_vec()
}

#[aoc(day15, part2)]
fn part2(input: &[(String, Op)]) -> usize {
    let mut boxes: [Vec<(String, FocalLength)>; 256] = std::array::from_fn(|_| Vec::new());
    for (label, op) in input {
        let box_index: usize = hash(label.chars()).into();
        let box_ = &mut boxes[box_index];
        match *op {
            Op::Dash => {
                box_.retain(|(l, _)| l != label);
            }
            Op::Equals(focal_length) => {
                let found_matching_label = box_
                    .iter()
                    .enumerate()
                    .find_map(|(i, (l, _))| (l == label).then_some(i));
                match found_matching_label {
                    Some(i) => {
                        box_[i] = (label.clone(), focal_length);
                    }
                    None => {
                        box_.push((label.clone(), focal_length));
                    }
                }
            }
        }
    }

    boxes
        .into_iter()
        .enumerate()
        .flat_map(|(boxnum, box_)| {
            box_.into_iter()
                .enumerate()
                .map(move |(index_in_box, (_, focal_length))| {
                    (1 + boxnum) * (1 + index_in_box) * usize::from(focal_length)
                })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_hash() {
        assert_eq!(hash("HASH".chars()), 52);
    }

    #[test]
    fn part1_sample() {
        assert_eq!(
            part1(&parse(
                "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
"
            )),
            1320
        )
    }
}
