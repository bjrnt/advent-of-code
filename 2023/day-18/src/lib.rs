use itertools::Itertools;
use Direction::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_delta(self) -> (i64, i64) {
        match self {
            Up => (0, -1),
            Down => (0, 1),
            Left => (-1, 0),
            Right => (1, 0),
        }
    }
}

fn parse_input(input: &str) -> Vec<(Direction, i64)> {
    input
        .lines()
        .map(|l| {
            let (dir, digits, _) = l.split(' ').collect_tuple().unwrap();
            (
                match dir {
                    "U" => Up,
                    "D" => Down,
                    "L" => Left,
                    "R" => Right,
                    _ => unreachable!(),
                },
                digits.parse::<i64>().unwrap(),
            )
        })
        .collect_vec()
}

fn parse_hexadecimals(input: &str) -> Vec<(Direction, i64)> {
    input
        .lines()
        .map(|l| {
            let hex_digits = &l[l.len() - 7..l.len()];
            let steps = i64::from_str_radix(&hex_digits[0..hex_digits.len() - 2], 16).unwrap();
            let dir = [Right, Down, Left, Up][usize::from_str_radix(
                &hex_digits[hex_digits.len() - 2..hex_digits.len() - 1],
                16,
            )
            .unwrap()];
            (dir, steps)
        })
        .collect_vec()
}

fn instructions_to_area(dirs_and_steps: &[(Direction, i64)]) -> i64 {
    let mut pts = Vec::new();
    pts.push((0, 0));

    let mut num_boundary_points = 0;
    let (mut cx, mut cy) = (0, 0);
    for (dir, steps) in dirs_and_steps.iter() {
        let (dx, dy) = dir.to_delta();
        num_boundary_points += steps;
        (cx, cy) = (cx + dx * steps, cy + dy * steps);
        pts.push((cx, cy));
    }

    // Shoelace formula, https://en.wikipedia.org/wiki/Shoelace_formula#Other_formulas_2
    let area = (0..pts.len() - 1)
        .map(|i| (pts[i].1 + pts[i + 1].1) * (pts[i].0 - pts[i + 1].0))
        .sum::<i64>()
        .abs()
        / 2;

    // Pick's theorem, https://en.wikipedia.org/wiki/Pick%27s_theorem
    let num_interior_points = area - num_boundary_points / 2 + 1;

    num_interior_points + num_boundary_points
}

pub fn process_part1(input: &str) -> String {
    instructions_to_area(&parse_input(input)).to_string()
}

pub fn process_part2(input: &str) -> String {
    instructions_to_area(&parse_hexadecimals(input)).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
        "62"
    )]
    #[trace]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part1(input).as_str(), expected);
    }

    #[rstest]
    #[case(
        "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
        "952408144115"
    )]
    #[trace]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part2(input).as_str(), expected);
    }
}
