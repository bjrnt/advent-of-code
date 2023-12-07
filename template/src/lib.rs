pub fn process_part1(_input: &str) -> String {
    "".to_string()
}

pub fn process_part2(_input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    const EXAMPLE_INPUT: &str = "";

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "");
    }

    #[test]
    #[ignore]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT), "");
    }

    #[rstest]
    #[case(0, 0)]
    #[trace]
    #[ignore]
    fn test_fn(#[case] input: u32, #[case] expected: u32) {
        assert_eq!(input, expected);
    }
}
