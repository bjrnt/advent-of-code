use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

fn pipe_neighbors(x: i32, y: i32, ch: char) -> Vec<(i32, i32)> {
    match ch {
        '|' => vec![(x, y - 1), (x, y + 1)],
        '-' => vec![(x - 1, y), (x + 1, y)],
        'L' => vec![(x, y - 1), (x + 1, y)],
        'F' => vec![(x, y + 1), (x + 1, y)],
        'J' => vec![(x, y - 1), (x - 1, y)],
        '7' => vec![(x, y + 1), (x - 1, y)],
        _ => vec![],
    }
}

pub fn process_part1(input: &str) -> String {
    let mut graph: HashMap<(i32, i32), char> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.char_indices() {
            graph.insert((x as i32, y as i32), char);
        }
    }

    let starting_position = *graph.iter().find(|(_, char)| **char == 'S').unwrap().0;
    let starting_pipes = graph
        .iter()
        .filter(|((x, y), ch)| pipe_neighbors(*x, *y, **ch).contains(&starting_position))
        .map(|(p, c)| (*p, *c, 1))
        .collect_vec();
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut queue = VecDeque::from(starting_pipes);
    let mut max_distance = 0;
    while let Some(((x, y), ch, distance)) = queue.pop_front() {
        if distance > max_distance {
            max_distance = distance;
        }

        for neighbor in pipe_neighbors(x, y, ch) {
            if let Some(ch) = graph.get(&neighbor) {
                if seen.insert(neighbor) {
                    queue.push_back((neighbor, *ch, distance + 1));
                }
            }
        }
    }

    max_distance.to_string()
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
        "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
        "4"
    )]
    #[case(
        "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
        "8"
    )]
    #[trace]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part1(input).as_str(), expected);
    }

    #[rstest]
    #[case(
        "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........",
        "4"
    )]
    #[trace]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part2(input).as_str(), expected);
    }
}
