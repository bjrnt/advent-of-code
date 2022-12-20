use std::collections::HashSet;

use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::newline, multi::separated_list1,
    sequence::separated_pair, IResult,
};

const SAND_SPAWN_POS: (u32, u32) = (500, 0);

fn parse_rocks(input: &str) -> IResult<&str, impl Iterator<Item = (u32, u32)>> {
    let (input, path_pairs) = separated_list1(
        newline,
        separated_list1(
            tag(" -> "),
            separated_pair(
                nom::character::complete::u32,
                tag(","),
                nom::character::complete::u32,
            ),
        ),
    )(input)?;

    // Have to use into_iter or the values won't live long enough
    let rocks_iter = path_pairs.into_iter().flat_map(|path| {
        path.into_iter()
            .tuple_windows()
            .flat_map(|((ax, ay), (bx, by))| {
                (ax.min(bx)..=ax.max(bx)).cartesian_product(ay.min(by)..=ay.max(by))
            })
    });

    Ok((input, rocks_iter))
}

pub fn process_part1(input: &str) -> String {
    let (_, rocks) = parse_rocks(input).unwrap();

    let mut grid: HashSet<(u32, u32)> = HashSet::from_iter(rocks);
    let rock_count = grid.len();

    let max_y = *grid.iter().map(|(_, y)| y).max().unwrap();

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
                break;
            }
        }

        // fallen by all rocks - no more sand will be stable
        if sand_y == max_y {
            break;
        }
    }

    (grid.len() - rock_count).to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, rocks) = parse_rocks(input).unwrap();

    let mut grid: HashSet<(u32, u32)> = HashSet::from_iter(rocks);
    let rock_count = grid.len();

    // floor is 2 below highest y, so stop at 1 past
    let max_y = 1 + grid.iter().map(|(_, y)| y).max().unwrap();

    loop {
        let (mut sand_x, mut sand_y) = SAND_SPAWN_POS;

        while sand_y < max_y {
            if grid.get(&(sand_x, sand_y + 1)).is_none() {
                sand_y += 1;
            } else if grid.get(&(sand_x - 1, sand_y + 1)).is_none() {
                sand_x -= 1;
                sand_y += 1;
            } else if grid.get(&(sand_x + 1, sand_y + 1)).is_none() {
                sand_x += 1;
                sand_y += 1;
            } else {
                grid.insert((sand_x, sand_y));
                break;
            }
        }

        if sand_y == max_y {
            // stop sand if it hits the floor
            grid.insert((sand_x, sand_y));
        } else if (sand_x, sand_y) == SAND_SPAWN_POS {
            break;
        }
    }

    (grid.len() - rock_count).to_string()
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
