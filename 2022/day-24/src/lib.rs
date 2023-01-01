use std::collections::HashMap;

use glam::IVec2;
use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator::{eof, iterator},
    multi::many1,
    sequence::terminated,
    IResult, Parser,
};
use petgraph::{algo::dijkstra, prelude::DiGraphMap};

#[derive(Debug, PartialEq, Clone)]
enum Cell {
    Wall,
    Blizzard(Vec<Direction>),
    Space,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn grid(input: &str) -> IResult<&str, HashMap<IVec2, Cell>> {
    let mut it = iterator(
        input,
        terminated(
            many1(alt((
                char('.').map(|_| Cell::Space),
                char('#').map(|_| Cell::Wall),
                char('v').map(|_| Cell::Blizzard(vec![Direction::Down])),
                char('^').map(|_| Cell::Blizzard(vec![Direction::Up])),
                char('<').map(|_| Cell::Blizzard(vec![Direction::Left])),
                char('>').map(|_| Cell::Blizzard(vec![Direction::Right])),
            ))),
            alt((line_ending, eof)),
        ),
    );
    let cells = it
        .enumerate()
        .flat_map(|(y, line)| {
            line.into_iter()
                .enumerate()
                .map(move |(x, cell)| (IVec2::new(x as i32, y as i32), cell))
        })
        .collect::<HashMap<IVec2, Cell>>();
    let res: IResult<_, _> = it.finish();
    Ok((res.unwrap().0, cells))
}

fn dimensions(grid: &HashMap<IVec2, Cell>) -> IVec2 {
    let x_max = grid.iter().map(|(v, _)| v.x).max().unwrap();
    let y_max = grid.iter().map(|(v, _)| v.y).max().unwrap();
    IVec2::new(x_max - 1, y_max - 1)
}

fn move_blizzard(
    grid: &HashMap<IVec2, Cell>,
    new_grid: &mut HashMap<IVec2, Cell>,
    position: &IVec2,
    direction: &Direction,
) {
    let movement = match direction {
        Direction::Left => IVec2::new(-1, 0),
        Direction::Right => IVec2::new(1, 0),
        Direction::Up => IVec2::new(0, -1),
        Direction::Down => IVec2::new(0, 1),
    };
    let desired_position = *position + movement;
    match grid.get(&desired_position) {
        Some(Cell::Wall) => {
            let wall_position = grid
                .iter()
                .find(|(ivec, cell)| {
                    let is_wall = cell == &&Cell::Wall;
                    let is_in_row_or_column = match direction {
                        Direction::Left => ivec.x > position.x && ivec.y == position.y,
                        Direction::Right => ivec.x < position.x && ivec.y == position.y,
                        Direction::Up => ivec.y > position.y && ivec.x == position.x,
                        Direction::Down => ivec.y < position.y && ivec.x == position.x,
                    };
                    is_wall && is_in_row_or_column
                })
                .unwrap()
                .0;
            let next_to_wall = *wall_position + movement;
            new_grid
                .entry(next_to_wall)
                .and_modify(|cell| {
                    if let Cell::Blizzard(directions) = cell {
                        directions.push(*direction);
                    }
                })
                .or_insert(Cell::Blizzard(vec![*direction]));
        }
        Some(_) => {
            new_grid
                .entry(desired_position)
                .and_modify(|cell| {
                    if let Cell::Blizzard(directions) = cell {
                        directions.push(*direction);
                    }
                })
                .or_insert(Cell::Blizzard(vec![*direction]));
        }
        None => {
            panic!("should not be none")
        }
    }
}

fn step(grid: &HashMap<IVec2, Cell>) -> HashMap<IVec2, Cell> {
    let mut new_grid = HashMap::new();

    let blizzards = grid.iter().filter_map(|(pos, cell)| match cell {
        Cell::Wall => None,
        Cell::Blizzard(d) => Some((pos, d)),
        Cell::Space => None,
    });
    for (position, directions) in blizzards {
        for direction in directions {
            move_blizzard(grid, &mut new_grid, position, direction);
        }
    }

    // copy walls
    grid.iter().for_each(|(pos, cell)| match cell {
        Cell::Wall => {
            new_grid.insert(*pos, cell.clone());
        }
        _ => (),
    });

    // spaces
    let total_size = dimensions(grid) + IVec2::new(2, 2);
    for (y, x) in (0..total_size.y).cartesian_product(0..total_size.x) {
        let pos = IVec2::new(x, y);
        if new_grid.get(&pos).is_none() {
            new_grid.insert(pos, Cell::Space);
        }
    }

    new_grid
}

pub fn process_part1(input: &str) -> String {
    let (_, mut grid) = grid(input).unwrap();
    let grid_dimensions = dimensions(&grid);
    let step_cycle_number = [
        (grid_dimensions.x..).step_by(grid_dimensions.x as usize),
        (grid_dimensions.y..).step_by(grid_dimensions.y as usize),
    ]
    .into_iter()
    .kmerge()
    .tuple_windows()
    .find(|(a, b)| a == b)
    .unwrap()
    .0;
    let end_position = grid
        .iter()
        .filter_map(|(pos, cell)| match cell {
            Cell::Wall => None,
            Cell::Blizzard(_) => None,
            Cell::Space => Some(pos),
        })
        .max_by(|IVec2 { x: x1, y: y1 }, IVec2 { x: x2, y: y2 }| (y1, x1).cmp(&(y2, x2)))
        .unwrap()
        .clone();

    let mut edges: Vec<((i32, i32, i32), (i32, i32, i32))> = vec![];
    for i in 0..step_cycle_number {
        let next_grid = step(&grid);
        let origin_spaces = grid.iter().filter_map(|(pos, cell)| match cell {
            Cell::Wall => None,
            Cell::Blizzard(_) => None,
            Cell::Space => Some(pos),
        });

        for origin_position in origin_spaces {
            let possibe_next_positions = vec![(-1, 0), (0, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter_map(|(y, x)| {
                    let next_position = IVec2::new(x, y) + *origin_position;
                    if let Some(Cell::Space) = next_grid.get(&next_position) {
                        Some(next_position)
                    } else {
                        None
                    }
                });
            for pos in possibe_next_positions {
                edges.push((
                    (origin_position.x, origin_position.y, i),
                    (
                        pos.x,
                        pos.y,
                        if i + 1 == step_cycle_number { 0 } else { i + 1 },
                    ),
                ));
            }
        }

        grid = next_grid;
    }

    let graph = DiGraphMap::<(i32, i32, i32), ()>::from_edges(edges);
    let result = dijkstra(&graph, (1, 0, 0), None, |_| 1);
    result
        .iter()
        .filter_map(|(end, value)| {
            if end.0 == end_position.x && end.1 == end_position.y {
                Some(value)
            } else {
                None
            }
        })
        .min()
        .unwrap()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, mut grid) = grid(input).unwrap();
    let grid_dimensions = dimensions(&grid);
    let step_cycle_number = [
        (grid_dimensions.x..).step_by(grid_dimensions.x as usize),
        (grid_dimensions.y..).step_by(grid_dimensions.y as usize),
    ]
    .into_iter()
    .kmerge()
    .tuple_windows()
    .find(|(a, b)| a == b)
    .unwrap()
    .0;
    let end_position = grid
        .iter()
        .filter_map(|(pos, cell)| match cell {
            Cell::Wall => None,
            Cell::Blizzard(_) => None,
            Cell::Space => Some(pos),
        })
        .max_by(|IVec2 { x: x1, y: y1 }, IVec2 { x: x2, y: y2 }| (y1, x1).cmp(&(y2, x2)))
        .unwrap()
        .clone();

    let mut edges: Vec<((i32, i32, i32), (i32, i32, i32))> = vec![];
    for i in 0..step_cycle_number {
        let next_grid = step(&grid);
        let origin_spaces = grid.iter().filter_map(|(pos, cell)| match cell {
            Cell::Wall => None,
            Cell::Blizzard(_) => None,
            Cell::Space => Some(pos),
        });

        for origin_position in origin_spaces {
            let possibe_next_positions = vec![(-1, 0), (0, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter_map(|(y, x)| {
                    let next_position = IVec2::new(x, y) + *origin_position;
                    if let Some(Cell::Space) = next_grid.get(&next_position) {
                        Some(next_position)
                    } else {
                        None
                    }
                });
            for pos in possibe_next_positions {
                edges.push((
                    (origin_position.x, origin_position.y, i),
                    (
                        pos.x,
                        pos.y,
                        if i + 1 == step_cycle_number { 0 } else { i + 1 },
                    ),
                ));
            }
        }

        grid = next_grid;
    }

    let graph = DiGraphMap::<(i32, i32, i32), ()>::from_edges(edges);
    let result = dijkstra(&graph, (1, 0, 0), None, |_| 1);
    let to_goal = result
        .iter()
        .filter(|(end, _)| end.0 == end_position.x && end.1 == end_position.y)
        .min_by_key(|(_, value)| *value)
        .unwrap();

    let result = dijkstra(&graph, *to_goal.0, None, |_| 1);
    let back_to_camp = result
        .iter()
        .filter(|(end, _)| end.0 == 1 && end.1 == 0)
        .min_by_key(|(_, value)| *value)
        .unwrap();

    let result = dijkstra(&graph, *back_to_camp.0, None, |_| 1);
    let back_to_goal = result
        .iter()
        .filter(|(end, _)| end.0 == end_position.x && end.1 == end_position.y)
        .min_by_key(|(_, value)| *value)
        .unwrap();

    (*to_goal.1 + *back_to_camp.1 + *back_to_goal.1).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
    const EXAMPLE_INPUTS: [(&str, &str, &str); 0] = [];

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "18");
    }

    #[test]
    fn part1_inputs() {
        for (input, answer_part1, _) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part1(input), answer_part1.to_string());
        }
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT), "54");
    }

    #[test]
    #[ignore]
    fn part2_inputs() {
        for (input, _, answer_part_2) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part2(input), answer_part_2.to_string());
        }
    }
}
