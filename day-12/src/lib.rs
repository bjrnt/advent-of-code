use std::collections::{HashMap, HashSet, VecDeque};

pub fn process_part1(input: &str) -> String {
    let grid_height = input.lines().count() as i32;
    let grid_width = input.find('\n').unwrap() as i32;

    let mut start = None;
    let mut end = None;
    let mut grid = HashMap::with_capacity((grid_height * grid_width) as usize);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
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
    let mut next_positions = VecDeque::from([(start, 0)]);

    while let Some(((cx, cy), steps)) = next_positions.pop_front() {
        if (cx, cy) == end {
            min_steps_to_goal = Some(steps);
            break;
        }
        let current_height = *grid.get(&(cx, cy)).unwrap();

        [(cx - 1, cy), (cx + 1, cy), (cx, cy - 1), (cx, cy + 1)]
            .iter()
            .filter(|&&(x, y)| {
                grid.get(&(x, y))
                    .map(|height| *height as i32 - current_height as i32 <= 1)
                    .unwrap_or(false)
            })
            .for_each(|neighbor| {
                if seen.insert(*neighbor) {
                    next_positions.push_back((*neighbor, steps + 1));
                }
            });
    }

    min_steps_to_goal.unwrap().to_string()
}

pub fn process_part2(input: &str) -> String {
    let grid_height = input.lines().count() as i32;
    let grid_width = input.find('\n').unwrap() as i32;

    let mut starts = vec![];
    let mut end = None;
    let mut grid = HashMap::with_capacity((grid_height * grid_width) as usize);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
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

    let mut next_positions = VecDeque::from_iter(starts.iter().map(|s| (*s, 0)));
    let mut seen: HashSet<(i32, i32)> = HashSet::from_iter(starts.into_iter());

    while let Some(((cx, cy), steps)) = next_positions.pop_front() {
        if (cx, cy) == end {
            return steps.to_string();
        }
        let current_height = *grid.get(&(cx, cy)).unwrap();

        [(cx - 1, cy), (cx + 1, cy), (cx, cy - 1), (cx, cy + 1)]
            .into_iter()
            .filter(|p| {
                grid.get(p)
                    .map(|height| *height as i32 - current_height as i32 <= 1)
                    .unwrap_or(false)
                    && seen.insert(*p)
            })
            .for_each(|neighbor| {
                next_positions.push_back((neighbor, steps + 1));
            });
    }
    return "no path found".to_string();
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
