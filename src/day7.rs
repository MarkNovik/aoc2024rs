use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::ops::{Add, Mul};

pub fn part1(input: &str) -> u64 {
    parse_input(input)
        .into_par_iter()
        .filter(|(res, elements)| can_be_made_with(*res, elements, &[u64::add, u64::mul], 0))
        .map(|(res, _)| res)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    parse_input(input)
        .into_par_iter()
        .filter(|(res, elements)| {
            can_be_made_with(*res, elements, &[u64::add, u64::mul, concat], 0)
        })
        .map(|(res, _)| res)
        .sum()
}

fn concat(a: u64, b: u64) -> u64 {
    (a.to_string() + &b.to_string()).parse().unwrap()
}

fn can_be_made_with(
    res: u64,
    elements: &[u64],
    operations: &[fn(u64, u64) -> u64],
    acc: u64,
) -> bool {
    if elements.is_empty() {
        res == acc
    } else {
        let (first, rest) = elements.split_first().unwrap();
        operations
            .iter()
            .any(|op| can_be_made_with(res, rest, operations, op(acc, *first)))
    }
}

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|l| {
            let (res, elements) = l.split_once(':').unwrap();
            (
                res.parse().unwrap(),
                elements
                    .split_whitespace()
                    .map(|s| s.trim().parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod test {
    const INPUT: &str = include_str!("./testInput/day7.txt");
    use super::part1;
    use super::part2;

    #[test]
    fn part1_test() {
        assert_eq!(3749, part1(INPUT));
    }

    #[test]
    fn part2_test() {
        assert_eq!(11387, part2(INPUT));
    }
}
