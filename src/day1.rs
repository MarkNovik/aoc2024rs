use std::collections::HashMap;
use std::iter::zip;

pub fn part1(input: &str) -> u64 {
    let (left, right) = parse_input(input);
    zip(left, right).map(|(a, b)| a.abs_diff(b)).sum()
}

pub fn part2(input: &str) -> u64 {
    let (left, right) = parse_input(input);
    let right = {
        let mut map = HashMap::new();
        for b in right {
            match map.get_mut(&b) {
                Some(value) => *value += 1,
                None => {
                    map.insert(b, 1);
                }
            }
        }
        map
    };
    left.into_iter()
        .map(|a| a * right.get(&a).copied().unwrap_or_default())
        .sum()
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .fold((vec![], vec![]), |(mut a, mut b), line| {
            let mut words = line.split_whitespace().map(|w| w.parse::<u64>().unwrap());
            a.push(words.next().unwrap());
            b.push(words.next().unwrap());
            (a, b)
        })
}

#[cfg(test)]
mod test {
    const INPUT: &str = include_str!("testInput/day1.txt");
    #[test]
    fn test_part1() {
        assert_eq!(11, super::part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(31, super::part2(INPUT));
    }
}
