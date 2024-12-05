use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

type Rules = HashMap<u64, HashSet<u64>>;
type Update = Vec<u64>;

pub fn part1(input: &str) -> u64 {
    let (rules, updates) = parse_input(input);
    updates
        .into_iter()
        .filter(|u| is_correct(&rules, u))
        .map(|u| u[u.len() / 2])
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let (rules, updates) = parse_input(input);
    updates
        .into_iter()
        .filter(|u| !is_correct(&rules, u))
        .map(|mut u| {
            u.sort_by(|a, b| {
                if let Some(true) = rules.get(a).map(|it| it.contains(b)) {
                    Ordering::Greater
                } else if let Some(true) = rules.get(b).map(|it| it.contains(a)) {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            });
            u[u.len() / 2]
        })
        .sum()
}

fn parse_input(input: &str) -> (Rules, Vec<Update>) {
    let (r, u) = input.split_once("\n\n").unwrap();
    let rules = {
        let mut rules = Rules::new();
        for line in r.lines() {
            let (before, after) = line
                .split('|')
                .map(|s| s.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap();
            if let Some(afters) = rules.get_mut(&before) {
                afters.insert(after);
            } else {
                rules.insert(before, HashSet::from([after]));
            }
        }
        rules
    };
    let updates = u
        .lines()
        .map(|s| s.split(',').map(|s| s.parse::<u64>().unwrap()).collect())
        .collect();
    (rules, updates)
}

fn is_correct(rules: &Rules, update: &Update) -> bool {
    rules.iter().all(|(k, v)| {
        let Some(ki) = update.iter().position(|i| i == k) else {
            return true;
        };
        v.iter()
            .filter_map(|a| update.iter().position(|i| i == a))
            .all(|i| i > ki)
    })
}

#[cfg(test)]
mod test {
    const INPUT: &str = include_str!("./testInput/day5.txt");

    #[test]
    fn part1_test() {
        assert_eq!(143, super::part1(INPUT));
    }

    #[test]
    fn part2_test() {
        assert_eq!(123, super::part2(INPUT));
    }
}
