pub fn process_part1(input: &str) -> String {
    "".to_string()
}

pub fn process_part2(input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "";
    const EXAMPLE_INPUTS: [(&str, &str, &str); 1] = [("", "", "")];

    #[test]
    fn part1() {
        assert_eq!(process_part1(EXAMPLE_INPUT), "");
    }

    #[test]
    #[ignore]
    fn part1_inputs() {
        for (input, answer_part1, _) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part1(input), answer_part1.to_string());
        }
    }

    #[test]
    #[ignore]
    fn part2() {
        assert_eq!(process_part2(EXAMPLE_INPUT), "");
    }

    #[test]
    #[ignore]
    fn part2_inputs() {
        for (input, _, answer_part_2) in EXAMPLE_INPUTS.iter() {
            assert_eq!(process_part2(input), answer_part_2.to_string());
        }
    }
}
