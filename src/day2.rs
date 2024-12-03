use itertools::Itertools;
use crate::drop_at::DropAt;

type Report = Vec<i64>;

pub fn part1(input: &str) -> usize {
    parse_reports(input).into_iter().filter(is_safe).count()
}

pub fn part2(input: &str) -> usize {
    parse_reports(input)
        .into_iter()
        .filter(is_safe_dampened)
        .count()
}

fn is_safe_dampened(report: &Report) -> bool {
    (0..report.len())
        .map(|i| report.clone().drop_at(i))
        .any(|r| is_safe(&r))
}

fn is_safe(report: &Report) -> bool {
    is_smooth(report) && is_consistent(report)
}

fn is_consistent(report: &Report) -> bool {
    report
        .iter()
        .tuple_windows()
        .map(|(a, b)| (a - b).signum())
        .tuple_windows()
        .map(|(a, b)| a != 0 && a == b)
        .all(|b| b)
}

fn is_smooth(report: &Report) -> bool {
    report
        .iter()
        .tuple_windows()
        .map(|(a, &b)| a.abs_diff(b))
        .all(|a| (1..=3).contains(&a))
}

fn parse_reports(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|w| w.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test {
    const INPUT: &str = include_str!("testInput/day2.txt");
    #[test]
    fn test_part1() {
        assert_eq!(2, super::part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(4, super::part2(INPUT));
    }
}
