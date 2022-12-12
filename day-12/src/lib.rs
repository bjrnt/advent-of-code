use std::collections::{HashMap, HashSet, VecDeque};

pub fn process_part1(input: &str) -> String {
    let grid_height = input.lines().count();
    let grid_width = input.find('\n').unwrap();

    let mut start = None;
    let mut end = None;
    let mut grid = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert(
                (x, y),
                match c {
                    'S' => {
                        start = Some((x, y));
                        0
                    }
                    'E' => {
                        end = Some((x, y));
                        'z' as u32 - 'a' as u32
                    }
                    c => c as u32 - 'a' as u32,
                },
            );
        }
    }

    let start = start.expect("couldn't find start");
    let end = end.expect("couldn't find end");

    let mut min_steps_to_goal = None;
    let mut seen = HashSet::from([(0, 0)]);
    let mut next_positions: VecDeque<((usize, usize), usize)> = VecDeque::from([(start, 0)]);
    while let Some(((cx, cy), steps)) = next_positions.pop_front() {
        if (cx, cy) == end {
            min_steps_to_goal = Some(steps);
            break;
        }
        let current_height = *grid.get(&(cx, cy)).unwrap();

        let mut neighbors = vec![];
        if cx >= 1 {
            neighbors.push((cx - 1, cy));
        }
        if cx < grid_width - 1 {
            neighbors.push((cx + 1, cy));
        }
        if cy >= 1 {
            neighbors.push((cx, cy - 1));
        }
        if cy < grid_height - 1 {
            neighbors.push((cx, cy + 1));
        }

        for neighbor in neighbors
            .iter()
            .filter(|neighbor| (*grid.get(neighbor).unwrap() as i32 - current_height as i32) <= 1)
        {
            if !seen.contains(neighbor) {
                next_positions.push_back((*neighbor, steps + 1));
                seen.insert(*neighbor);
            }
        }
    }

    min_steps_to_goal.unwrap().to_string()
}

pub fn process_part2(input: &str) -> String {
    let grid_height = input.lines().count();
    let grid_width = input.find('\n').unwrap();

    let mut starts = vec![];
    let mut end = None;
    let mut grid = HashMap::with_capacity(grid_height * grid_width);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert(
                (x, y),
                match c {
                    'S' | 'a' => {
                        starts.push((x, y));
                        0
                    }
                    'E' => {
                        end = Some((x, y));
                        'z' as u32 - 'a' as u32
                    }
                    c => c as u32 - 'a' as u32,
                },
            );
        }
    }

    let end = end.expect("couldn't find end");

    starts
        .iter()
        .filter_map(|start| {
            let mut seen: HashSet<(usize, usize)> = HashSet::from([*start]);
            let mut next_positions: VecDeque<((usize, usize), usize)> =
                VecDeque::from([(*start, 0)]);

            while let Some(((cx, cy), steps)) = next_positions.pop_front() {
                if (cx, cy) == end {
                    return Some(steps);
                }

                let current_height = *grid.get(&(cx, cy)).unwrap();

                let mut neighbors = vec![];
                if cx >= 1 {
                    neighbors.push((cx - 1, cy));
                }
                if cx < grid_width - 1 {
                    neighbors.push((cx + 1, cy));
                }
                if cy >= 1 {
                    neighbors.push((cx, cy - 1));
                }
                if cy < grid_height - 1 {
                    neighbors.push((cx, cy + 1));
                }

                for neighbor in neighbors.iter().filter(|neighbor| {
                    (*grid.get(neighbor).unwrap() as i32 - current_height as i32) <= 1
                }) {
                    if !seen.contains(neighbor) {
                        next_positions.push_back((*neighbor, steps + 1));
                        seen.insert(*neighbor);
                    }
                }
            }
            None
        })
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    const EXAMPLE_INPUTS: [(&str, &str, &str); 0] = [];

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "31");
    }

    #[test]
    fn part1_inputs() {
        for (input, answer_part1, _) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part1(input), answer_part1.to_string());
        }
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT), "29");
    }

    #[test]
    #[ignore]
    fn part2_inputs() {
        for (input, _, answer_part_2) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part2(input), answer_part_2.to_string());
        }
    }
}
