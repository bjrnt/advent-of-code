pub fn process_part1(_input: &str) -> String {
    "".to_string()
}

pub fn process_part2(_input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("", "")]
    #[trace]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part1(input).as_str(), expected);
    }

    #[rstest]
    #[case("", "")]
    #[trace]
    #[ignore]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part2(input).as_str(), expected);
    }
}
