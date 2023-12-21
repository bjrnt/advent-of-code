use std::collections::{HashMap, HashSet, VecDeque};

fn parse_input(input: &str) -> ((i32, i32), HashSet<(i32, i32)>) {
    let mut rocks = HashSet::new();
    let mut starting_position = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.char_indices() {
            if ch == '#' {
                rocks.insert((x as i32, y as i32));
            } else if ch == 'S' {
                starting_position = (x as i32, y as i32);
            }
        }
    }
    (starting_position, rocks)
}

pub fn process_part1(input: &str) -> String {
    let (start, rocks) = parse_input(input);
    let (max_x, max_y) = aoc_utils::grid_bounds(input);
    let (max_x, max_y) = (max_x as i32, max_y as i32);
    let num_steps = if max_x < 20 { 6 } else { 64 };

    let mut visited = HashMap::new();
    let mut q = VecDeque::new();
    q.push_back((start, 0));

    while let Some(((cx, cy), steps)) = q.pop_front() {
        if steps > num_steps {
            break;
        }

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)].into_iter() {
            let (nx, ny) = (cx + dx, cy + dy);
            if nx >= 0
                && nx < max_x
                && ny >= 0
                && ny < max_y
                && !rocks.contains(&(nx, ny))
                && visited.insert((nx, ny), steps + 1).is_none()
            {
                q.push_back(((nx, ny), steps + 1));
            }
        }
    }

    visited
        .into_values()
        .filter(|s| s % 2 == 0)
        .count()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (start, rocks) = parse_input(input);
    let (max_x, max_y) = aoc_utils::grid_bounds(input);
    let (max_x, max_y) = (max_x as i32, max_y as i32);

    let mut visited = HashMap::new();
    let mut q = VecDeque::new();
    q.push_back((start, 0));

    while let Some(((cx, cy), steps)) = q.pop_front() {
        if !visited.insert((cx, cy), steps).is_none() {
            continue;
        }
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)].into_iter() {
            let (nx, ny) = (cx + dx, cy + dy);
            if nx >= 0
                && nx < max_x
                && ny >= 0
                && ny < max_y
                && !rocks.contains(&(nx, ny))
                && !visited.contains_key(&(nx, ny))
            {
                q.push_back(((nx, ny), steps + 1));
            }
        }
    }

    let even_corners = visited
        .values()
        .filter(|s| **s % 2 == 0 && **s > 65)
        .count();
    let odd_corners = visited
        .values()
        .filter(|s| **s % 2 == 1 && **s > 65)
        .count();

    let n = (26501365 - max_x as usize / 2) / max_y as usize;
    assert_eq!(n, 202300);

    let even = n * n;
    let odd = (n + 1) * (n + 1);

    let total = odd * visited.values().filter(|s| **s % 2 == 1).count()
        + even * visited.values().filter(|s| **s % 2 == 0).count()
        - ((n + 1) * odd_corners)
        + (n * even_corners);
    total.to_string()
}
