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

pub fn process_part2(input: &str) -> String {
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
    let mut queue = VecDeque::from(starting_pipes.clone());
    while let Some(((x, y), ch, distance)) = queue.pop_front() {
        for neighbor in pipe_neighbors(x, y, ch) {
            if let Some(ch) = graph.get(&neighbor) {
                if seen.insert(neighbor) {
                    queue.push_back((neighbor, *ch, distance + 1));
                }
            }
        }
    }

    let starting_pipes_positions = starting_pipes.iter().map(|(p, _, _)| p).collect_vec();
    let potential_starting_characters = vec!['|', '-', 'L', 'J', 'F', '7'];
    let starting_character = potential_starting_characters
        .iter()
        .find(|symbol| {
            pipe_neighbors(starting_position.0, starting_position.1, **symbol)
                .iter()
                .all(|p| starting_pipes_positions.contains(&p))
        })
        .unwrap();
    graph.insert(starting_position, *starting_character);
    graph = graph
        .into_iter()
        .map(|(p, c)| if seen.contains(&p) { (p, c) } else { (p, '.') })
        .collect();
    (0..input.lines().count())
        .map(|y| ray_trace(y as i32, &graph))
        .sum::<usize>()
        .to_string()
}

fn ray_trace(y: i32, graph: &HashMap<(i32, i32), char>) -> usize {
    let flip_on_chars = vec!['F', '|', '7'];
    let mut x = 0;
    let mut is_inside = false;
    let mut symbols_inside = 0;
    while let Some(ch) = graph.get(&(x, y)) {
        if flip_on_chars.contains(ch) {
            is_inside = !is_inside;
        } else if *ch == '.' && is_inside {
            symbols_inside += 1;
        }
        x += 1;
    }
    symbols_inside
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
    #[case(
        ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        "8"
    )]
    #[case(
        "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        "10"
    )]
    #[trace]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part2(input).as_str(), expected);
    }
}
