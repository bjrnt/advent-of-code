use std::collections::{BinaryHeap, HashMap};
use Direction::*;

#[derive(Debug, PartialEq, Eq, Clone)]
struct State((i32, i32), Direction, u32);

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.2.cmp(&self.2))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.2.cmp(&self.2)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn_cw(&self) -> Direction {
        match &self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }

    pub fn turn_ccw(&self) -> Direction {
        match &self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        }
    }

    pub fn delta(&self) -> (i32, i32) {
        match &self {
            Up => (0, -1),
            Down => (0, 1),
            Left => (-1, 0),
            Right => (1, 0),
        }
    }
}

fn next_steps(
    grid: &HashMap<(i32, i32), u32>,
    heat_so_far: u32,
    (x, y): (i32, i32),
    prev_dir: Direction,
    is_ultra_crucible: bool,
) -> Vec<((i32, i32), Direction, u32)> {
    let r_start = if is_ultra_crucible { 4 } else { 1 };
    let r_end = if is_ultra_crucible { 10 } else { 3 };
    let mut next_states = vec![];
    {
        let ld = prev_dir.turn_ccw();
        let (lx, ly) = ld.delta();
        let mut lh = heat_so_far;
        for total_steps in 1..=r_end {
            let lp = (x + lx * total_steps, y + ly * total_steps);
            if let Some(h) = grid.get(&lp) {
                lh += h;
                if total_steps >= r_start {
                    next_states.push((lp, ld, lh));
                }
            } else {
                break;
            }
        }
    }
    {
        let rd = prev_dir.turn_cw();
        let (rx, ry) = rd.delta();
        let mut rh = heat_so_far;
        for total_steps in 1..=r_end {
            let rp = (x + rx * total_steps, y + ry * total_steps);
            if let Some(h) = grid.get(&rp) {
                rh += h;
                if total_steps >= r_start {
                    next_states.push((rp, rd, rh));
                }
            } else {
                break;
            }
        }
    }
    next_states
}

fn parse_grid(input: &str) -> HashMap<(i32, i32), u32> {
    let mut map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, d) in line.char_indices() {
            map.insert((x as i32, y as i32), d.to_digit(10).unwrap());
        }
    }
    map
}

fn djikstras(
    grid: &HashMap<(i32, i32), u32>,
    (max_x, max_y): (usize, usize),
    is_ultra_crucible: bool,
) -> u32 {
    let mut min_heat = HashMap::new();
    min_heat.insert(((0, 0), Up), 0);

    let mut heap = BinaryHeap::new();
    heap.push(State((0, 0), Up, 0));
    heap.push(State((0, 0), Left, 0));

    while let Some(State(p, prev_dir, heat_loss)) = heap.pop() {
        if p == (max_x as i32 - 1, max_y as i32 - 1) {
            return heat_loss;
        }

        let best_so_far = min_heat.entry((p, prev_dir)).or_insert(u32::MAX);
        if heat_loss > *best_so_far {
            continue;
        }

        for (p, d, h) in next_steps(grid, heat_loss, p, prev_dir, is_ultra_crucible) {
            let best_so_far = min_heat.entry((p, d)).or_insert(u32::MAX);
            if h < *best_so_far {
                *best_so_far = h;
                heap.push(State(p, d, h));
            }
        }
    }
    0
}

pub fn process_part1(input: &str) -> String {
    djikstras(&parse_grid(input), aoc_utils::grid_bounds(input), false).to_string()
}

pub fn process_part2(input: &str) -> String {
    djikstras(&parse_grid(input), aoc_utils::grid_bounds(input), true).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
        "102"
    )]
    #[trace]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part1(input).as_str(), expected);
    }

    #[rstest]
    #[case(
        "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
        "94"
    )]
    #[case(
        "111111111111
999999999991
999999999991
999999999991
999999999991",
        "71"
    )]
    #[trace]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part2(input).as_str(), expected);
    }
}
