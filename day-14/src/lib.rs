use std::collections::HashSet;

use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::newline, multi::separated_list1,
    sequence::separated_pair, IResult,
};

mod utils;
use utils::*;

const SAND_SPAWN_POS: (u32, u32) = (500, 0);

fn parse_paths(input: &str) -> IResult<&str, Vec<Vec<(u32, u32)>>> {
    separated_list1(
        newline,
        separated_list1(
            tag(" -> "),
            separated_pair(
                nom::character::complete::u32,
                tag(","),
                nom::character::complete::u32,
            ),
        ),
    )(input)
}

fn straight_line(from: &(u32, u32), to: &(u32, u32)) -> Vec<(u32, u32)> {
    if from.1 != to.1 {
        let (min_y, max_y) = minmax(from.1, to.1);
        (min_y..=max_y).map(|y| (from.0, y)).collect::<Vec<_>>()
    } else {
        let (min_x, max_x) = minmax(from.0, to.0);
        ((min_x..=max_x).map(|x| (x, from.1))).collect::<Vec<_>>()
    }
}

pub fn process_part1(input: &str) -> String {
    let (_, rock_paths) = parse_paths(input).unwrap();

    let mut grid: HashSet<(u32, u32)> = HashSet::from_iter(
        rock_paths
            .iter()
            .flat_map(|path| path.iter().tuple_windows::<(&(u32, u32), &(u32, u32))>())
            .flat_map(|(from, to)| straight_line(from, to)),
    );

    let max_y = rock_paths.iter().flatten().map(|p| p.1).max().unwrap();
    let mut sand_count = 0;

    loop {
        let (mut sand_x, mut sand_y) = SAND_SPAWN_POS;

        while sand_y < max_y {
            if grid.get(&(sand_x, sand_y + 1)).is_none() {
                sand_y += 1;
            } else if grid.get(&(sand_x - 1, sand_y + 1)).is_none() {
                sand_y += 1;
                sand_x -= 1;
            } else if grid.get(&(sand_x + 1, sand_y + 1)).is_none() {
                sand_y += 1;
                sand_x += 1;
            } else {
                grid.insert((sand_x, sand_y));
                sand_count += 1;
                break;
            }
        }

        // fallen by all rocks - no more sand will be stable
        if sand_y == max_y {
            break;
        }
    }
    sand_count.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, rock_paths) = parse_paths(input).unwrap();

    let mut grid: HashSet<(u32, u32)> = HashSet::from_iter(
        rock_paths
            .iter()
            .flat_map(|path| path.iter().tuple_windows::<(&(u32, u32), &(u32, u32))>())
            .flat_map(|(from, to)| straight_line(from, to)),
    );

    let max_y = 2 + rock_paths.iter().flatten().map(|p| p.1).max().unwrap();
    let mut sand_count = 0;

    loop {
        let (mut sand_x, mut sand_y) = SAND_SPAWN_POS;

        while sand_y < max_y {
            if grid.get(&(sand_x, sand_y + 1)).is_none() {
                sand_y += 1;
            } else if grid.get(&(sand_x - 1, sand_y + 1)).is_none() {
                sand_y += 1;
                sand_x -= 1;
            } else if grid.get(&(sand_x + 1, sand_y + 1)).is_none() {
                sand_y += 1;
                sand_x += 1;
            } else {
                grid.insert((sand_x, sand_y));
                sand_count += 1;
                break;
            }
        }

        if (sand_x, sand_y) == SAND_SPAWN_POS {
            break;
        }
    }
    sand_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    const EXAMPLE_INPUTS: [(&str, &str, &str); 0] = [];

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "24");
    }

    #[test]
    fn part1_inputs() {
        for (input, answer_part1, _) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part1(input), answer_part1.to_string());
        }
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT), "93");
    }

    #[test]
    #[ignore]
    fn part2_inputs() {
        for (input, _, answer_part_2) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part2(input), answer_part_2.to_string());
        }
    }
}
