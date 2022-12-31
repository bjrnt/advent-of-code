use std::collections::{HashMap, HashSet};

use glam::IVec2;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use nom::{
    branch::alt,
    character::complete::{line_ending, one_of},
    combinator::{eof, iterator},
    multi::many1,
    sequence::terminated,
    IResult,
};

fn elves(input: &str) -> IResult<&str, HashSet<IVec2>> {
    let mut it = iterator(
        input,
        terminated(many1(one_of(".#")), alt((line_ending, eof))),
    );
    let elves = it
        .enumerate()
        .flat_map(|(y, line)| {
            line.into_iter()
                .enumerate()
                .filter_map(move |(x, c)| match c {
                    '.' => None,
                    '#' => Some(IVec2::new(x as i32, y as i32)),
                    _ => panic!("unknown char"),
                })
        })
        .collect::<HashSet<IVec2>>();
    let res: IResult<_, _> = it.finish();
    Ok((res.unwrap().0, elves))
}

pub fn process_part1(input: &str) -> String {
    let (_, mut elves) = elves(input).unwrap();
    let checks = vec![
        [IVec2::new(-1, -1), IVec2::new(0, -1), IVec2::new(1, -1)],
        [IVec2::new(-1, 1), IVec2::new(0, 1), IVec2::new(1, 1)],
        [IVec2::new(-1, -1), IVec2::new(-1, 0), IVec2::new(-1, 1)],
        [IVec2::new(1, -1), IVec2::new(1, 0), IVec2::new(1, 1)],
    ];
    let checks_iter = checks.iter().cycle();

    for i in 0..10 {
        let local_checks = checks_iter.clone().skip(i).take(4);
        let mut proposed_moves: HashMap<IVec2, Vec<IVec2>> = HashMap::new();

        for elf in elves.iter() {
            // check for all empty around elf
            if local_checks
                .clone()
                .flat_map(|v| v.iter().map(|vec| *vec + *elf))
                .unique()
                .all(|value| elves.get(&value).is_none())
            {
                proposed_moves
                    .entry(*elf)
                    // .and_modify(|value| value.push(*elf))
                    .or_insert(vec![*elf]);
                continue;
            }
            // check for a possible move in a direction
            let possible_move = local_checks.clone().find_map(|checks| {
                checks
                    .iter()
                    .all(|position| elves.get(&(*position + *elf)).is_none())
                    .then_some(checks[1] + *elf)
            });
            if let Some(r#move) = possible_move {
                proposed_moves
                    .entry(r#move)
                    .and_modify(|value| value.push(*elf))
                    .or_insert(vec![*elf]);
            } else {
                proposed_moves
                    .entry(*elf)
                    // .and_modify(|value| value.push(*elf))
                    .or_insert(vec![*elf]);
            }
        }

        elves = proposed_moves
            .into_iter()
            .flat_map(|(desired_position, elves_to_move)| {
                if elves_to_move.len() == 1 {
                    vec![desired_position]
                } else {
                    elves_to_move
                }
            })
            .collect::<HashSet<IVec2>>();
    }

    let minmax_x = elves.iter().map(|v| v.x).minmax();
    let minmax_y = elves.iter().map(|v| v.y).minmax();

    let (MinMax(x1,x2), MinMax(y1,y2)) = (minmax_x, minmax_y) else {
        panic!("")
    };

    let min_box_size = (x2 - x1 + 1) * (y2 - y1 + 1);
    (min_box_size as usize - elves.len()).to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, mut elves) = elves(input).unwrap();
    let checks = vec![
        [IVec2::new(-1, -1), IVec2::new(0, -1), IVec2::new(1, -1)],
        [IVec2::new(-1, 1), IVec2::new(0, 1), IVec2::new(1, 1)],
        [IVec2::new(-1, -1), IVec2::new(-1, 0), IVec2::new(-1, 1)],
        [IVec2::new(1, -1), IVec2::new(1, 0), IVec2::new(1, 1)],
    ];
    let checks_iter = checks.iter().cycle();

    let mut rounds = 0;

    for i in 0.. {
        let local_checks = checks_iter.clone().skip(i).take(4);
        let mut proposed_moves: HashMap<IVec2, Vec<IVec2>> = HashMap::new();

        for elf in elves.iter() {
            // check for all empty around elf
            if local_checks
                .clone()
                .flat_map(|v| v.iter().map(|vec| *vec + *elf))
                .unique()
                .all(|value| elves.get(&value).is_none())
            {
                proposed_moves
                    .entry(*elf)
                    // .and_modify(|value| value.push(*elf))
                    .or_insert(vec![*elf]);
                continue;
            }
            // check for a possible move in a direction
            let possible_move = local_checks.clone().find_map(|checks| {
                checks
                    .iter()
                    .all(|position| elves.get(&(*position + *elf)).is_none())
                    .then_some(checks[1] + *elf)
            });
            if let Some(r#move) = possible_move {
                proposed_moves
                    .entry(r#move)
                    .and_modify(|value| value.push(*elf))
                    .or_insert(vec![*elf]);
            } else {
                proposed_moves
                    .entry(*elf)
                    // .and_modify(|value| value.push(*elf))
                    .or_insert(vec![*elf]);
            }
        }

        let new_elves = proposed_moves
            .into_iter()
            .flat_map(|(desired_position, elves_to_move)| {
                if elves_to_move.len() == 1 {
                    vec![desired_position]
                } else {
                    elves_to_move
                }
            })
            .collect::<HashSet<IVec2>>();

        if elves == new_elves {
            rounds = i;
            break;
        } else {
            elves = new_elves;
        }
    }

    (rounds + 1).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
    const EXAMPLE_INPUTS: [(&str, &str, &str); 0] = [];

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "110");
    }

    #[test]
    fn part1_inputs() {
        for (input, answer_part1, _) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part1(input), answer_part1.to_string());
        }
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT), "20");
    }

    #[test]
    #[ignore]
    fn part2_inputs() {
        for (input, _, answer_part_2) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part2(input), answer_part_2.to_string());
        }
    }
}
