pub fn part1(input: &str) -> usize {
    let mat = parse_input(input);
    let slopes = [
        Offset(0, 1),
        Offset(0, -1),
        Offset(1, 0),
        Offset(-1, 0),
        Offset(1, 1),
        Offset(1, -1),
        Offset(-1, 1),
        Offset(-1, -1),
    ];
    mat.cells()
        .filter(|(_, c)| c == &'X')
        .map(|(pos, _)| {
            slopes
                .iter()
                .filter(|slope| has_word_at(&mat, pos, **slope, "XMAS"))
                .count()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mat = parse_input(input);
    mat.cells()
        .filter(|(pos, _)| has_mas_cross_at(&mat, *pos))
        .count()
}

fn parse_input(input: &str) -> Matrix {
    Matrix {
        lines: input.lines().map(|x| x.chars().collect()).collect(),
    }
}

fn has_mas_cross_at(mat: &Matrix, pos: Pos) -> bool {
    let has_mas_at = |start: Offset| {
        let Some(m) = pos + start else { return false };
        let Some(s) = pos - start else { return false };
        let ms = mat.get(m) == Some('M') && mat.get(s) == Some('S');
        let sm = mat.get(m) == Some('S') && mat.get(s) == Some('M');
        ms || sm
    };

    if let Some('A') = mat.get(pos) {
        has_mas_at(Offset(1, 1)) && has_mas_at(Offset(1, -1))
    } else {
        false
    }
}

fn has_word_at(mat: &Matrix, pos: Pos, slope: Offset, looking_for: &str) -> bool {
    match (mat.get(pos), looking_for.chars().next()) {
        (a, b) if looking_for.chars().count() == 1 => a == b,
        (Some(a), Some(b)) => {
            let Some(next_pos) = pos + slope else { return false };
            a == b && has_word_at(mat, next_pos, slope, &looking_for[b.len_utf8()..])
        }
        (_, None) | (None, _) => false,
    }
}

#[derive(Copy, Clone)]
struct Pos(usize, usize);

#[derive(Copy, Clone)]
struct Offset(i64, i64);
impl std::ops::Add<Offset> for Pos {
    type Output = Option<Pos>;
    fn add(self, rhs: Offset) -> Self::Output {
        Some(Pos(
            usize::try_from(i128::try_from(self.0).ok()? + i128::from(rhs.0)).ok()?,
            usize::try_from(i128::try_from(self.1).ok()? + i128::from(rhs.1)).ok()?,
        ))
    }
}

impl std::ops::Sub<Offset> for Pos {
    type Output = Option<Pos>;
    fn sub(self, rhs: Offset) -> Self::Output {
        Some(Pos(
            usize::try_from(i128::try_from(self.0).ok()? - i128::from(rhs.0)).ok()?,
            usize::try_from(i128::try_from(self.1).ok()? - i128::from(rhs.1)).ok()?,
        ))
    }
}
struct Matrix {
    lines: Vec<Vec<char>>,
}

impl Matrix {
    pub fn get(&self, Pos(x, y): Pos) -> Option<char> {
        self.lines.get(y)?.get(x).copied()
    }

    pub fn cells(&self) -> impl Iterator<Item = (Pos, char)> + '_ {
        self.lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, &c)| (Pos(x, y), c)))
    }
}

#[cfg(test)]
mod test {
    const INPUT: &str = include_str!("testInput/day4.txt");
    #[test]
    fn test_part1() {
        assert_eq!(18, super::part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(9, super::part2(INPUT));
    }
}