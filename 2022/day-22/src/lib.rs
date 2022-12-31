use std::collections::HashMap;

use glam::UVec2;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, one_of},
    combinator::iterator,
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult, Parser,
};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug)]
enum Move {
    Paces(u32),
    Turn(Turn),
}

impl Direction {
    fn turn(self, turn: &Turn) -> Direction {
        use Direction::*;
        match (self, turn) {
            (Up, Turn::Left) => Left,
            (Up, Turn::Right) => Right,
            (Down, Turn::Left) => Right,
            (Down, Turn::Right) => Left,
            (Left, Turn::Left) => Down,
            (Left, Turn::Right) => Up,
            (Right, Turn::Left) => Up,
            (Right, Turn::Right) => Down,
        }
    }
}

fn moves(input: &str) -> IResult<&str, Vec<Move>> {
    many1(alt((
        alt((
            tag("L").map(|_| Move::Turn(Turn::Left)),
            tag("R").map(|_| Move::Turn(Turn::Right)),
        )),
        complete::u32.map(|n| Move::Paces(n)),
    )))(input)
}

#[derive(Debug)]
enum Cell {
    Space,
    Wall,
}

#[derive(Debug)]
struct Field(HashMap<UVec2, Cell>);

impl Field {
    fn get_row(&self, target_y: u32) -> Vec<(&UVec2, &Cell)> {
        self.0
            .iter()
            .filter(|(UVec2 { y, .. }, _)| y == &target_y)
            .sorted_by(|(vec_a, _), (vec_b, _)| vec_a.x.cmp(&vec_b.x))
            .collect()
    }

    fn get_column(&self, target_x: u32) -> Vec<(&UVec2, &Cell)> {
        self.0
            .iter()
            .filter(|(UVec2 { x, .. }, _)| x == &target_x)
            .sorted_by(|(vec_a, _), (vec_b, _)| vec_a.y.cmp(&vec_b.y))
            .collect()
    }
}

fn field(input: &str) -> IResult<&str, Field> {
    let mut it = iterator(input, terminated(many1(one_of(" .#")), line_ending));
    let parsed: HashMap<UVec2, Cell> = it
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .filter_map(move |(x, cell)| match cell {
                    ' ' => None,
                    '.' => Some((UVec2::new(x as u32, y as u32), Cell::Space)),
                    '#' => Some((UVec2::new(x as u32, y as u32), Cell::Wall)),
                    _ => panic!("invalid character"),
                })
        })
        .collect();
    let res: IResult<_, _> = it.finish();
    res.map(|(input, _)| (input, Field(parsed)))
}

fn field_and_moves(input: &str) -> IResult<&str, (Field, Vec<Move>)> {
    separated_pair(field, line_ending, moves)(input)
}

fn pace<'a>(
    current_position: &mut UVec2,
    paces_to_move: u32,
    positions: impl Iterator<Item = &'a (&'a UVec2, &'a Cell)> + Clone,
) {
    let current_index = positions
        .clone()
        .position(|(vec, _)| vec == &current_position)
        .unwrap();
    let mut it = positions.cycle();
    it.nth(current_index);

    for _ in 1..=paces_to_move {
        let next_cell = it.next().unwrap();
        if let Cell::Wall = next_cell.1 {
            break;
        } else {
            current_position.x = next_cell.0.x;
            current_position.y = next_cell.0.y;
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let (_, (field, moves)) = field_and_moves(input).unwrap();

    let mut facing = Direction::Right;
    let starting_position = field
        .0
        .iter()
        .sorted_by(|(UVec2 { x: x1, y: y1 }, _), (UVec2 { x: x2, y: y2 }, _)| {
            (y1, x1).cmp(&(y2, x2))
        })
        .next()
        .unwrap();
    let mut current_position = *starting_position.0;

    for m in moves {
        match m {
            Move::Paces(paces_to_move) => {
                match facing {
                    Direction::Up => {
                        let x = current_position.x;
                        pace(
                            &mut current_position,
                            paces_to_move,
                            field.get_column(x).iter().rev(),
                        );
                    }
                    Direction::Down => {
                        let x = current_position.x;
                        pace(
                            &mut current_position,
                            paces_to_move,
                            field.get_column(x).iter(),
                        );
                    }
                    Direction::Left => {
                        let y = current_position.y;
                        pace(
                            &mut current_position,
                            paces_to_move,
                            field.get_row(y).iter().rev(),
                        );
                    }
                    Direction::Right => {
                        let y = current_position.y;
                        pace(
                            &mut current_position,
                            paces_to_move,
                            field.get_row(y).iter(),
                        );
                    }
                };
            }
            Move::Turn(turn) => facing = facing.turn(&turn),
        }
    }

    (1000 * (current_position.y + 1)
        + 4 * (current_position.x + 1)
        + match facing {
            Direction::Up => 3,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 0,
        })
    .to_string()
}

pub fn process_part2(input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../test.txt");
    const EXAMPLE_INPUTS: [(&str, &str, &str); 0] = [];

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "6032");
    }

    #[test]
    fn part1_inputs() {
        for (input, answer_part1, _) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part1(input), answer_part1.to_string());
        }
    }

    #[test]
    #[ignore]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT), "");
    }

    #[test]
    #[ignore]
    fn part2_inputs() {
        for (input, _, answer_part_2) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part2(input), answer_part_2.to_string());
        }
    }
}
