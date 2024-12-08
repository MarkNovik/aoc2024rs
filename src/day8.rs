use itertools::{chain, Itertools};
use std::collections::HashMap;
use std::iter::successors;
use std::ops::{Add, Sub};

pub fn part1(input: &str) -> usize {
    let (width, height, map) = parse_input(input);
    map.into_values()
        .flat_map(|positions| {
            positions
                .into_iter()
                .tuple_combinations()
                .flat_map(|(a, b)| {
                    let m = b - a;
                    [
                        Some(a - m).filter(|p| p.in_bound(width, height)),
                        Some(b + m).filter(|p| p.in_bound(width, height)),
                    ]
                    .into_iter()
                    .flatten()
                })
        })
        .unique()
        .count()
}

pub fn part2(input: &str) -> usize {
    let (width, height, map) = parse_input(input);
    map.into_values()
        .flat_map(|positions| {
            positions
                .into_iter()
                .tuple_combinations()
                .flat_map(|(a, b)| {
                    let m = b - a;
                    chain(
                        successors(Some(a), move |&it| Some(it - m))
                            .take_while(|p| p.in_bound(width, height)),
                        successors(Some(b), move |&it| Some(it + m))
                            .take_while(|p| p.in_bound(width, height)),
                    )
                })
        })
        .unique()
        .count()
}

fn parse_input(input: &str) -> (isize, isize, HashMap<char, Vec<Position>>) {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();
    let antennas = input
        .lines()
        .zip(0..)
        .flat_map(|(line, y)| {
            line.chars().zip(0..).filter_map(move |(ch, x)| {
                if ch.is_alphanumeric() {
                    Some((ch, Position(x, y)))
                } else {
                    None
                }
            })
        })
        .into_group_map();
    (width as isize, height as isize, antennas)
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Position(isize, isize);

impl Position {
    fn in_bound(&self, w: isize, h: isize) -> bool {
        self.0 >= 0 && self.0 < w && self.1 >= 0 && self.1 < h
    }
}

impl Sub<Position> for Position {
    type Output = Position;

    fn sub(self, other: Position) -> Self {
        Position(self.0 - other.0, self.1 - other.1)
    }
}

impl Add<Position> for Position {
    type Output = Position;
    fn add(self, other: Position) -> Self {
        Position(self.0 + other.0, self.1 + other.1)
    }
}

#[cfg(test)]
mod test {
    const INPUT: &str = include_str!("testInput/day8.txt");

    #[test]
    fn part1_test() {
        assert_eq!(14, super::part1(INPUT));
    }

    #[test]
    fn part2_test() {
        assert_eq!(34, super::part2(INPUT));
    }
}
