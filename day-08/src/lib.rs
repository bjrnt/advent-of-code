use std::collections::{HashMap, HashSet};

pub fn process_part1(input: &str) -> String {
    let len = input.find('\n').unwrap();
    let mut grid = HashMap::new();
    input.lines().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, height)| {
            grid.insert((x, y), height.to_digit(10));
        });
    });

    let mut visible = HashSet::new();

    for y in 0..len {
        let mut tallest_seen_so_far = grid.get(&(0, y)).unwrap();
        visible.insert((0, y));
        for x in 0..len {
            let pos = (x, y);
            let current_height = grid.get(&pos).unwrap();
            if current_height > tallest_seen_so_far {
                visible.insert(pos);
                tallest_seen_so_far = current_height;
            }
        }
    }
    for y in 0..len {
        let mut tallest_seen_so_far = grid.get(&(len - 1, y)).unwrap();
        visible.insert((len - 1, y));
        for x in (0..len - 1).rev() {
            let pos = (x, y);
            let current_height = grid.get(&pos).unwrap();
            if current_height > tallest_seen_so_far {
                visible.insert(pos);
                tallest_seen_so_far = current_height;
            }
        }
    }
    for x in 0..len {
        let mut tallest_seen_so_far = grid.get(&(x, 0)).unwrap();
        visible.insert((x, 0));
        for y in 0..len {
            let pos = (x, y);
            let current_height = grid.get(&pos).unwrap();
            if current_height > tallest_seen_so_far {
                visible.insert(pos);
                tallest_seen_so_far = current_height;
            }
        }
    }
    for x in 0..len {
        let mut tallest_seen_so_far = grid.get(&(x, len - 1)).unwrap();
        visible.insert((x, len - 1));
        for y in (0..len - 1).rev() {
            let pos = (x, y);
            let current_height = grid.get(&pos).unwrap();
            if current_height > tallest_seen_so_far {
                visible.insert(pos);
                tallest_seen_so_far = current_height;
            }
        }
    }
    visible.len().to_string()
}

pub fn process_part2(input: &str) -> String {
    let len = input.find('\n').unwrap();
    let mut grid = HashMap::new();
    input.lines().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, height)| {
            grid.insert((x, y), height.to_digit(10));
        });
    });

    let mut scores = HashMap::new();

    for y_h in 1..len - 1 {
        for x_h in 1..len - 1 {
            let house_height = grid.get(&(x_h, y_h)).unwrap();

            let mut x_inc_score = 0;
            for x in x_h + 1..len {
                let pos = (x, y_h);
                let current_height = grid.get(&pos).unwrap();
                x_inc_score += 1;
                if current_height >= house_height {
                    break;
                }
            }

            let mut x_dec_score = 0;
            for x in (0..x_h).rev() {
                let pos = (x, y_h);
                let current_height = grid.get(&pos).unwrap();
                x_dec_score += 1;
                if current_height >= house_height {
                    break;
                }
            }

            let mut y_inc_score = 0;
            for y in y_h + 1..len {
                let pos = (x_h, y);
                let current_height = grid.get(&pos).unwrap();
                y_inc_score += 1;
                if current_height >= house_height {
                    break;
                }
            }

            let mut y_dec_score = 0;
            for y in (0..y_h).rev() {
                let pos = (x_h, y);
                let current_height = grid.get(&pos).unwrap();
                y_dec_score += 1;
                if current_height >= house_height {
                    break;
                }
            }

            scores.insert(
                (x_h, y_h),
                x_inc_score * x_dec_score * y_inc_score * y_dec_score,
            );
        }
    }

    let max_score = scores.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
    max_score.1.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "30373
25512
65332
33549
35390";
    const EXAMPLE_INPUTS: [(&str, &str, &str); 0] = [];

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "21");
    }

    #[test]
    fn part1_inputs() {
        for (input, answer_part1, _) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part1(input), answer_part1.to_string());
        }
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT), "8");
    }

    #[test]
    fn part2_inputs() {
        for (input, _, answer_part_2) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part2(input), answer_part_2.to_string());
        }
    }
}
