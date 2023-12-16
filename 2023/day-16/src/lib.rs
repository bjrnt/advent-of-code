use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use rayon::prelude::*;
use Direction::*;
use Mirrors::*;
use Splitters::*;
use Tile::*;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Mirrors {
    Backslash,    // \
    Forwardslash, // /
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Splitters {
    Vertical,
    Horizontal,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Mirror(Mirrors),
    Splitter(Splitters),
}

fn parse_input(input: &str) -> HashMap<(i32, i32), Tile> {
    let mut tiles = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.char_indices() {
            let tile = match ch {
                '|' => Some(Splitter(Vertical)),
                '-' => Some(Splitter(Horizontal)),
                '/' => Some(Mirror(Forwardslash)),
                '\\' => Some(Mirror(Backslash)),
                _ => None,
            };
            if let Some(tile) = tile {
                tiles.insert((x as i32, y as i32), tile);
            };
        }
    }
    tiles
}

fn next_states(
    (x, y): (i32, i32),
    colliding_tile: Option<&Tile>,
    dir: Direction,
) -> Vec<((i32, i32), Direction)> {
    match (dir, colliding_tile) {
        // Straight
        (Up, None) | (Up, Some(Splitter(Vertical))) => vec![((x, y - 1), dir)],
        (Down, None) | (Down, Some(Splitter(Vertical))) => vec![((x, y + 1), dir)],
        (Left, None) | (Left, Some(Splitter(Horizontal))) => vec![((x - 1, y), dir)],
        (Right, None) | (Right, Some(Splitter(Horizontal))) => vec![((x + 1, y), dir)],
        // Turns
        (Up, Some(Mirror(Backslash))) | (Down, Some(Mirror(Forwardslash))) => {
            vec![((x - 1, y), Left)]
        }
        (Up, Some(Mirror(Forwardslash))) | (Down, Some(Mirror(Backslash))) => {
            vec![((x + 1, y), Right)]
        }
        (Left, Some(Mirror(Forwardslash))) | (Right, Some(Mirror(Backslash))) => {
            vec![((x, y + 1), Down)]
        }
        (Left, Some(Mirror(Backslash))) | (Right, Some(Mirror(Forwardslash))) => {
            vec![((x, y - 1), Up)]
        }
        // Splits
        (Up, Some(Splitter(Horizontal))) | (Down, Some(Splitter(Horizontal))) => {
            vec![((x - 1, y), Left), ((x + 1, y), Right)]
        }
        (Left, Some(Splitter(Vertical))) | (Right, Some(Splitter(Vertical))) => {
            vec![((x, y - 1), Up), ((x, y + 1), Down)]
        }
    }
}

fn is_within_bounds((max_x, max_y): (usize, usize), (x, y): (i32, i32)) -> bool {
    x >= 0 && x < max_x as i32 && y >= 0 && y < max_y as i32
}

fn start_to_energized(
    tiles: &HashMap<(i32, i32), Tile>,
    bounds: (usize, usize),
    start: ((i32, i32), Direction),
) -> usize {
    let mut seen: HashSet<((i32, i32), Direction)> = HashSet::new();
    seen.insert(start);
    aoc_utils::complete_bfs(vec![start].into_iter(), |((x, y), dir)| {
        let states = next_states((x, y), tiles.get(&(x, y)), dir);
        // println!(
        // "({:?}, {:?}) + {:?} -> {:?}",
        // (x, y),
        // dir,
        // tiles.get(&(x, y)),
        // &states
        // );
        Some(
            states
                .into_iter()
                .filter(|(p, d)| is_within_bounds(bounds, *p) && seen.insert((*p, *d)))
                .collect_vec(),
        )
    });
    // let m: HashMap<_, _> = seen.into_iter().collect();
    // for y in 0..bounds.1 {
    //     for x in 0..bounds.0 {
    //         if m.contains_key(&(x as i32, y as i32)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     print!("\n");
    // }
    seen.into_iter().unique_by(|(p, _)| *p).count()
}

pub fn process_part1(input: &str) -> String {
    let tiles = parse_input(input);
    let bounds = aoc_utils::grid_bounds(input);
    let start = ((0, 0), Right);
    start_to_energized(&tiles, bounds, start).to_string()
}

pub fn process_part2(input: &str) -> String {
    let tiles = parse_input(input);
    let (max_x, max_y) = aoc_utils::grid_bounds(input);
    let mut potential_starts = vec![];
    for x in 0..max_x {
        potential_starts.push(((x as i32, 0), Down));
        potential_starts.push(((x as i32, max_y as i32 - 1), Up));
    }
    for y in 0..max_y {
        potential_starts.push(((0, y as i32), Right));
        potential_starts.push(((max_x as i32 - 1, y as i32), Left));
    }
    potential_starts
        .into_par_iter()
        .map(|start| start_to_energized(&tiles, (max_x, max_y), start))
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
        "46"
    )]
    #[trace]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part1(input).as_str(), expected);
    }

    #[rstest]
    #[case(
        r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
        "51"
    )]
    #[trace]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part2(input).as_str(), expected);
    }
}
