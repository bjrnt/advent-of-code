use std::collections::HashSet;

use either::Either;
use itertools::Itertools;
use Direction::*;

fn parse_input(input: &str) -> (Vec<(usize, usize)>, HashSet<(usize, usize)>) {
    let mut movable = Vec::new();
    let mut fixed = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.char_indices() {
            match ch {
                '#' => {
                    fixed.insert((x, y));
                }
                'O' => movable.push((x, y)),
                _ => continue,
            };
        }
    }
    (movable, fixed)
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn tilt(
    movable: &[(usize, usize)],
    fixed: &HashSet<(usize, usize)>,
    dir: Direction,
    (max_x, max_y): (usize, usize),
) -> Vec<(usize, usize)> {
    let mut moved = HashSet::new();
    let (delta_x, delta_y) = match dir {
        North => (0, -1),
        South => (0, 1),
        East => (1, 0),
        West => (-1, 0),
    };
    let movable_it = movable.iter().sorted_by_key(|(x, y)| match dir {
        North | South => y,
        East | West => x,
    });
    let movable_it = if dir == South || dir == East {
        Either::Left(movable_it.rev())
    } else {
        Either::Right(movable_it)
    };
    for (mx, my) in movable_it {
        let (mut nx, mut ny) = (*mx, *my);
        while nx as i64 + delta_x >= 0
            && ((nx as i64 + delta_x) as usize) < max_x
            && (ny as i64 + delta_y) >= 0
            && ((ny as i64 + delta_y) as usize) < max_y
            && !moved.contains(&(
                (nx as i64 + delta_x) as usize,
                (ny as i64 + delta_y) as usize,
            ))
            && !fixed.contains(&(
                (nx as i64 + delta_x) as usize,
                (ny as i64 + delta_y) as usize,
            ))
        {
            nx = (nx as i64 + delta_x) as usize;
            ny = (ny as i64 + delta_y) as usize;
        }
        moved.insert((nx, ny));
    }
    // HashSet will be iterated in an arbitrary order, so sort first!
    moved.into_iter().sorted().collect()
}

fn cycle(
    movable: &[(usize, usize)],
    fixed: &HashSet<(usize, usize)>,
    bounds: (usize, usize),
) -> Vec<(usize, usize)> {
    [North, West, South, East]
        .into_iter()
        .fold(movable.to_vec(), |movable, dir| {
            tilt(&movable, fixed, dir, bounds)
        })
}

pub fn process_part1(input: &str) -> String {
    let (max_x, max_y) = aoc_utils::grid_bounds(input);
    let (movable, fixed) = parse_input(input);
    tilt(&movable, &fixed, North, (max_x, max_y))
        .into_iter()
        .map(|(_, y)| max_y - y)
        .sum::<usize>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (max_x, max_y) = aoc_utils::grid_bounds(input);
    let (mut movable, fixed) = parse_input(input);

    let mut seen: HashSet<Vec<(usize, usize)>> = HashSet::new();
    seen.insert(movable.clone());

    let mut states = vec![movable.clone()];

    let mut it = 0;
    loop {
        it += 1;
        movable = cycle(&movable, &fixed, (max_x, max_y));
        if seen.contains(&movable) {
            break;
        }
        seen.insert(movable.clone());
        states.push(movable.clone());
    }

    let start_of_loop_index = states.iter().position(|s| s == &movable).unwrap();
    let final_state_index =
        (1_000_000_000 - start_of_loop_index) % (it - start_of_loop_index) + start_of_loop_index;

    states[final_state_index]
        .iter()
        .map(|(_, y)| max_y - *y)
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part1() {
        assert_eq!(process_part1(INPUT).as_str(), "136");
    }

    #[rstest]
    #[case(
        1,
        ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
    )]
    #[case(
        2,
        ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"
    )]
    #[case(
        3,
        ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
    )]
    #[trace]
    fn test_cycle(#[case] cycles: usize, #[case] expected: &str) {
        let max_y = INPUT.lines().count();
        let max_x = INPUT.lines().next().unwrap().len();
        let (mut movable, fixed) = parse_input(INPUT);
        for _ in 0..cycles {
            movable = cycle(&movable, &fixed, (max_x, max_y));
        }
        let actual = grid_to_string(&movable, &fixed, (max_x, max_y));
        println!("{}", actual);
        println!("-----");
        println!("{}", expected);
        assert_eq!(actual.as_str(), expected);
    }

    #[rstest]
    #[case(
        North,
        "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."
    )]
    #[trace]
    fn test_tilt(#[case] dir: Direction, #[case] expected: &str) {
        let max_y = INPUT.lines().count();
        let max_x = INPUT.lines().next().unwrap().len();
        let (mut movable, fixed) = parse_input(INPUT);
        movable = tilt(&movable, &fixed, dir, (max_x, max_y));
        let actual = grid_to_string(&movable, &fixed, (max_x, max_y));
        println!("{}", actual);
        println!("-----");
        println!("{}", expected);
        assert_eq!(actual.as_str(), expected);
    }

    #[test]
    fn test_part2() {
        assert_eq!(process_part2(INPUT).as_str(), "64");
    }

    fn grid_to_string(
        movable: &[(usize, usize)],
        fixed: &HashSet<(usize, usize)>,
        (max_x, max_y): (usize, usize),
    ) -> String {
        let movable: HashSet<(usize, usize)> = movable.to_owned().into_iter().collect();
        let mut s = String::with_capacity(max_x * max_y);
        for y in 0..max_y {
            for x in 0..max_x {
                if movable.contains(&(x, y)) {
                    s.push('O');
                } else if fixed.contains(&(x, y)) {
                    s.push('#')
                } else {
                    s.push('.')
                }
            }
            s.push('\n')
        }
        s.trim().to_owned()
    }
}
