use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn apply(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }
}

fn move_tail((hx, hy): (i32, i32), (tx, ty): (i32, i32)) -> (i32, i32) {
    let (dx, dy) = ((hx - tx), (hy - ty));
    if dx.abs() > 1 || dy.abs() > 1 {
        (tx + dx.signum(), ty + dy.signum())
    } else {
        (tx, ty)
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Direction, u32)>> {
    let (input, moves) = separated_list1(
        newline,
        separated_pair(
            alpha1.map(|a: &str| match a {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("unrecognized direction"),
            }),
            tag(" "),
            digit1.map(|d: &str| d.parse::<u32>().unwrap()),
        ),
    )(input)?;
    Ok((input, moves))
}

pub fn process_part1(input: &str) -> String {
    let (_, head_moves) = parse_input(input).unwrap();
    let mut head_position = (0, 0);
    let mut unique_tail_positions = HashSet::from([(0, 0)]);
    let mut tail_position = (0, 0);
    for dir in head_moves
        .iter()
        .flat_map(|(dir, steps)| vec![*dir; *steps as usize])
    {
        head_position = dir.apply(head_position);
        tail_position = move_tail(head_position, tail_position);
        unique_tail_positions.insert(tail_position);
    }
    unique_tail_positions.len().to_string()
}

pub fn process_part2(input: &str) -> String {
    const ROPE_LENGTH: usize = 10;
    let (_, head_moves) = parse_input(input).unwrap();
    let mut unique_last_knot_positions = HashSet::from([(0, 0)]);
    let mut rope = [(0, 0); ROPE_LENGTH];
    for dir in head_moves
        .iter()
        .flat_map(|(dir, steps)| vec![*dir; *steps as usize])
    {
        rope[0] = dir.apply(rope[0]);
        for i in 1..ROPE_LENGTH {
            rope[i] = move_tail(rope[i - 1], rope[i])
        }
        unique_last_knot_positions.insert(rope[ROPE_LENGTH - 1]);
    }
    unique_last_knot_positions.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    const LARGER_EXAMPLE_INPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    const EXAMPLE_INPUTS: [(&str, &str, &str); 0] = [];

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "13");
    }

    #[test]
    fn part1_inputs() {
        for (input, answer_part1, _) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part1(input), answer_part1.to_string());
        }
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT), "1");
    }

    #[test]
    fn part2_2() {
        assert_eq!(process_part2(LARGER_EXAMPLE_INPUT), "36");
    }

    #[test]
    #[ignore]
    fn part2_inputs() {
        for (input, _, answer_part_2) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part2(input), answer_part_2.to_string());
        }
    }

    #[test]
    fn test_tail() {
        assert_eq!((0, 0), move_tail((0, 0), (0, 0)));
        assert_eq!((0, 0), move_tail((1, 1), (0, 0)));
        assert_eq!((0, 1), move_tail((0, 2), (0, 0)));
        assert_eq!((0, -1), move_tail((0, -2), (0, 0)));
        assert_eq!((1, 0), move_tail((2, 0), (0, 0)));
        assert_eq!((-1, 0), move_tail((-2, 0), (0, 0)));
        assert_eq!((1, 1), move_tail((2, 1), (0, 0)))
    }
}
