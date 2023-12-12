use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn parse_image(input: &str) -> Vec<(i32, i32)> {
    let mut image: Vec<(i32, i32)> = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                image.push((x as i32, y as i32));
            }
        }
    }
    image
}

fn expand_image(image: &Vec<(i32, i32)>, max_x: i32, max_y: i32) -> Vec<(i32, i32)> {
    let empty_rows = (0..max_y)
        .filter(|y| !image.iter().any(|(_, gy)| gy == y))
        .collect_vec();
    let mut expanded = image.clone();
    for empty_row in empty_rows {
        expanded = expanded
            .iter()
            .map(|&(gx, gy)| {
                if gy > empty_row {
                    (gx, gy + 1)
                } else {
                    (gx, gy)
                }
            })
            .collect_vec();
    }
    let empty_columns = (0..max_x)
        .filter(|x| !image.iter().any(|(gx, _)| gx == x))
        .collect_vec();
    for empty_column in empty_columns {
        expanded = expanded
            .iter()
            .map(|&(gx, gy)| {
                if gx > empty_column {
                    (gx + 1, gy)
                } else {
                    (gx, gy)
                }
            })
            .collect_vec();
    }
    expanded
}

pub fn process_part1(input: &str) -> String {
    let mut image = parse_image(input);
    let max_y = input.lines().count() as i32;
    let max_x = input.lines().next().unwrap().chars().count() as i32;
    let expanded_image = expand_image(&mut image, max_x, max_y);
    expanded_image
        .into_iter()
        .combinations(2)
        .map(|galaxy_pair| {
            let [(x1, y1), (x2, y2)] = galaxy_pair.as_slice() else {
                panic!("unexpected galaxy pair")
            };
            dbg!(&galaxy_pair, (x1 - x2).abs() + (y1 - y2).abs());
            (x1 - x2).abs() + (y1 - y2).abs()
        })
        .sum::<i32>()
        .to_string()
}

pub fn process_part2(_input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        "374"
    )]
    #[trace]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part1(input).as_str(), expected);
    }

    #[rstest]
    #[case("", "")]
    #[trace]
    #[ignore]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part2(input).as_str(), expected);
    }
}
