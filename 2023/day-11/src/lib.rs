use itertools::Itertools;

fn parse_image(input: &str) -> Vec<(usize, usize)> {
    let mut image: Vec<(usize, usize)> = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                image.push((x, y));
            }
        }
    }
    image
}

fn expand_image(image: &[(usize, usize)], expansion_factor: usize) -> Vec<(usize, usize)> {
    let max_x = image.iter().max_by_key(|(gx, _)| *gx).unwrap().0;
    let max_y = image.iter().max_by_key(|(_, gy)| *gy).unwrap().1;
    let empty_rows = (0..max_y)
        .filter(|y| image.iter().all(|(_, gy)| gy != y))
        .collect_vec();
    let empty_cols = (0..max_x)
        .filter(|x| image.iter().all(|(gx, _)| gx != x))
        .collect_vec();
    image
        .iter()
        .map(|&(gx, gy)| {
            (
                gx + empty_cols.iter().filter(|c| gx > **c).count() * (expansion_factor - 1),
                gy + empty_rows.iter().filter(|c| gy > **c).count() * (expansion_factor - 1),
            )
        })
        .collect_vec()
}

pub fn process_part1(input: &str) -> String {
    expand_image(&parse_image(input), 2)
        .into_iter()
        .combinations(2)
        .map(|galaxy_pair| {
            let [(x1, y1), (x2, y2)] = galaxy_pair.as_slice() else {
                panic!("unexpected galaxy pair: {:?}", galaxy_pair)
            };
            (*x1 as i64 - *x2 as i64).unsigned_abs() + (*y1 as i64 - *y2 as i64).unsigned_abs()
        })
        .sum::<u64>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    expand_image(&parse_image(input), 1_000_000)
        .into_iter()
        .combinations(2)
        .map(|galaxy_pair| {
            let [(x1, y1), (x2, y2)] = galaxy_pair.as_slice() else {
                panic!("unexpected galaxy pair: {:?}", galaxy_pair)
            };
            (*x1 as i64 - *x2 as i64).unsigned_abs() + (*y1 as i64 - *y2 as i64).unsigned_abs()
        })
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "#.#
        ...
#.#",
        "24"
    )]
    #[case(
        "#
.
.
#", "5"
    )]
    #[case("#..#", "5")]
    #[case(
        "#..
...
..#",
        "6"
    )]
    #[case(
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        "374"
    )]
    #[trace]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part1(input).as_str(), expected);
    }
}
