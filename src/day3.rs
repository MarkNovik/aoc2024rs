#[derive(Debug)]
enum Command {
    Do,
    Dont,
    Mul(usize, usize),
}

pub fn part1(input: &str) -> usize {
    parse_input(input)
        .into_iter()
        .map(|c| if let Command::Mul(a, b) = c { a * b } else { 0 })
        .sum()
}

pub fn part2(input: &str) -> usize {
    parse_input(input)
        .into_iter()
        .fold((0, true), |(acc, active), next| match next {
            Command::Do => (acc, true),
            Command::Dont => (acc, false),
            Command::Mul(a, b) => {
                if active {
                    (acc + a * b, active)
                } else {
                    (acc, active)
                }
            }
        })
        .0
}

fn parse_input(mut input: &str) -> Vec<Command> {
    fn shift(input: &str) -> &str {
        if let Some(c) = input.chars().next() {
            &input[c.len_utf8()..]
        } else {
            input
        }
    }
    fn matches<'a>(input: &'a str, prefix: &str) -> (&'a str, bool) {
        if let Some(rest) = input.strip_prefix(prefix) {
            (rest, true)
        } else {
            (input, false)
        }
    }
    fn match_mul(input: &str) -> Option<(&str, Command)> {
        let (rest, m) = input
            .strip_prefix("mul(")
            .and_then(|rest| rest.split_once(','))
            .and_then(|(a, rest)| {
                if let Ok(num) = a.parse::<usize>() {
                    rest.split_once(')').map(|(a, b)| (num, a, b))
                } else {
                    None
                }
            })
            .and_then(|(numa, b, rest)| {
                if let Ok(numb) = b.parse::<usize>() {
                    Some((rest, Command::Mul(numa, numb)))
                } else {
                    None
                }
            })?;
        Some((rest, m))
    }
    let mut result = Vec::new();
    while !input.is_empty() {
        if let (rest, true) = matches(input, "do()") {
            input = rest;
            result.push(Command::Do);
        } else if let (rest, true) = matches(input, "don't()") {
            input = rest;
            result.push(Command::Dont);
        } else if let Some((rest, m)) = match_mul(input) {
            input = rest;
            result.push(m);
        } else {
            input = shift(input);
        }
    }
    result
}

#[cfg(test)]
mod test {
    const INPUT_P1: &str = include_str!("testInput/day3p1.txt");
    const INPUT_P2: &str = include_str!("testInput/day3p2.txt");
    #[test]
    fn test_part1() {
        assert_eq!(161, super::part1(INPUT_P1));
    }

    #[test]
    fn test_part2() {
        assert_eq!(48, super::part2(INPUT_P2));
    }
}
