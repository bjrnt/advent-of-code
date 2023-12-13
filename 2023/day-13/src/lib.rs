use itertools::Itertools;

fn parse_patterns(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec()
        })
        .collect_vec()
}

fn find_reflection(pattern: &Vec<Vec<char>>) -> (usize, usize) {
    for col in 1..pattern[0].len() {
        let mut left = col as i64 - 1;
        let mut right = col;
        let mut valid = true;
        while left >= 0 && right < pattern[0].len() {
            if get_column(pattern, left as usize) != get_column(pattern, right) {
                valid = false;
                break;
            }
            left -= 1;
            right += 1;
        }
        if valid {
            return (0, col);
        }
    }
    for row in 1..pattern.len() {
        let mut up = row as i64 - 1;
        let mut down = row;
        let mut valid = true;
        while up >= 0 && down < pattern.len() {
            if pattern[up as usize] != pattern[down] {
                valid = false;
                break;
            }
            up -= 1;
            down += 1;
        }
        if valid {
            return (row, 0);
        }
    }
    (0, 0)
}

fn find_reflection_with_smudge(pattern: &Vec<Vec<char>>) -> (usize, usize) {
    for col in 1..pattern[0].len() {
        let mut left = col as i64 - 1;
        let mut right = col;
        let mut differences = 0;
        while left >= 0 && right < pattern[0].len() {
            differences += get_column(pattern, left as usize).iter().zip(get_column(pattern, right)).filter(|(l,r)| *l != r).count();
            if differences > 1 {
                break
            }
            left -= 1;
            right += 1;
        }
        if differences == 1 {
            return (0, col);
        }
    }
    for row in 1..pattern.len() {
        let mut up = row as i64 - 1;
        let mut down = row;
        let mut differences = 0;
        while up >= 0 && down < pattern.len() {
            differences += pattern[up as usize].iter().zip(pattern[down].iter()).filter(|(l,r)| l != r).count();
            if differences > 1 {
                break
            }
            up -= 1;
            down += 1;
        }
        if differences == 1 {
            return (row, 0);
        }
    }
    (0, 0)
}

fn get_column(pattern: &Vec<Vec<char>>, column: usize) -> Vec<char> {
    pattern.iter().map(|row| row[column]).collect_vec()
}

pub fn process_part1(input: &str) -> String {
    let patterns = parse_patterns(input);
    patterns
        .iter()
        .map(|pattern| {
            let (horizontal, vertical) = find_reflection(pattern);
            horizontal * 100 + vertical
        })
        .sum::<usize>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let patterns = parse_patterns(input);
    patterns
        .iter()
        .map(|pattern| {
            let (horizontal, vertical)= find_reflection_with_smudge(pattern);
            horizontal * 100 + vertical
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        "5"
    )]
    #[case(
        "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        "400"
    )]
    #[trace]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part1(input).as_str(), expected);
    }

    #[rstest]
    #[case(
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        "300"
    )]
    #[case(
        "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        "100"
    )]
    #[trace]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part2(input).as_str(), expected);
    }
}
