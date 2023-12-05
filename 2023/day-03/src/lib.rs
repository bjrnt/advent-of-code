use std::collections::{HashMap, HashSet};

fn part_number(grid: &HashMap<(i32, i32), char>, (x, y): (i32, i32)) -> ((i32, i32), u32) {
    let mut start_x = x;
    while grid
        .get(&(start_x - 1, y))
        .is_some_and(|c| c.is_ascii_digit())
    {
        start_x -= 1;
    }
    let mut end_x = x;
    while grid
        .get(&(end_x + 1, y))
        .is_some_and(|c| c.is_ascii_digit())
    {
        end_x += 1;
    }

    let mut num = 0;
    for x in start_x..=end_x {
        num = num * 10 + grid.get(&(x, y)).unwrap().to_digit(10).unwrap();
    }
    ((start_x, y), num)
}

pub fn process_part1(input: &str) -> String {
    let mut grid = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert((x as i32, y as i32), c);
        }
    }

    let positions_of_symbols = grid
        .iter()
        .filter_map(|(p, c)| {
            if c.is_ascii_digit() || c == &'.' {
                None
            } else {
                Some(*p)
            }
        })
        .collect::<Vec<(i32, i32)>>();

    let mut reachable_from_symbol = HashSet::new();
    let mut numbers = vec![];

    for &(cx, cy) in positions_of_symbols.iter() {
        let neighbors = vec![
            (cx - 1, cy - 1),
            (cx - 1, cy),
            (cx - 1, cy + 1),
            (cx, cy - 1),
            (cx, cy + 1),
            (cx + 1, cy - 1),
            (cx + 1, cy),
            (cx + 1, cy + 1),
        ];

        neighbors.into_iter().for_each(|neighbor| {
            if grid.get(&neighbor).is_some_and(|c| c.is_ascii_digit()) {
                let (origin, value) = part_number(&grid, neighbor);
                if reachable_from_symbol.insert(origin) {
                    numbers.push(value);
                }
            }
        });
    }
    numbers.iter().sum::<u32>().to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut grid = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert((x as i32, y as i32), c);
        }
    }

    let positions_of_symbols = grid
        .iter()
        .filter_map(|(p, c)| {
            if c.is_ascii_digit() || c == &'.' {
                None
            } else {
                Some(*p)
            }
        })
        .collect::<Vec<(i32, i32)>>();

    let mut reachable_from_symbol = HashSet::new();
    let mut gear_ratios = vec![];

    for &(cx, cy) in positions_of_symbols.iter() {
        let neighbors = vec![
            (cx - 1, cy - 1),
            (cx - 1, cy),
            (cx - 1, cy + 1),
            (cx, cy - 1),
            (cx, cy + 1),
            (cx + 1, cy - 1),
            (cx + 1, cy),
            (cx + 1, cy + 1),
        ];

        let neighbor_numbers = neighbors
            .into_iter()
            .filter_map(|neighbor| {
                if grid.get(&neighbor).is_some_and(|c| c.is_ascii_digit()) {
                    let (origin, value) = part_number(&grid, neighbor);
                    if reachable_from_symbol.insert(origin) {
                        return Some(value);
                    };
                };
                None
            })
            .collect::<Vec<u32>>();

        if grid.get(&(cx, cy)).is_some_and(|c| c == &'*') && neighbor_numbers.len() == 2 {
            gear_ratios.push(neighbor_numbers.iter().product());
        }
    }
    gear_ratios.iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+..58
..592.....
......755.
...$.*....
.664.598..";
    const EXAMPLE_INPUTS: [(&str, &str, &str); 0] = [];

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "4361");
    }

    #[test]
    fn part1_number_at_end() {
        assert_eq!(
            process_part1(
                "...12
12*.."
            ),
            "24"
        );
    }

    #[test]
    fn part1_number_under() {
        assert_eq!(
            process_part1(
                "..11.
..*.."
            ),
            "11"
        );
    }

    #[test]
    fn part1_symbol_in_middle() {
        assert_eq!(process_part1("..50+50.."), "100");
    }

    #[test]
    fn part1_symbol_diagonals() {
        assert_eq!(
            process_part1(
                "1.1
.+.
1.1"
            ),
            "4"
        );
    }

    #[test]
    fn part1_symbol_not_diagonal() {
        assert_eq!(
            process_part1(
                ".1.
1+1
.1."
            ),
            "4"
        );
    }

    #[test]
    fn part1_other_input() {
        assert_eq!(
            process_part1(
                ".......................*......*
...910*...............233..189.
2......391.....789*............
...................983.........
0........106-...............226
.%............................$
...*......$812......812..851...
.99.711.............+.....*....
...........................113.
28*.....411....%..............."
            ),
            "7253"
        );
    }

    #[test]
    fn part1_inputs() {
        for (input, answer_part1, _) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part1(input), answer_part1.to_string());
        }
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT), "467835");
    }

    #[test]
    #[ignore]
    fn part2_inputs() {
        for (input, _, answer_part_2) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part2(input), answer_part_2.to_string());
        }
    }
}
