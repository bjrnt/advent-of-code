use std::collections::HashMap;

use nom::IResult;

enum Step {
    Left,
    Right
}

impl TryFrom<char> for Step {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Step::Left),
            'R' => Ok(Step::Right),
            _ => Err(())
        }
    }
}

fn parse_graph(input: &str) -> IResult<&str, (Vec<Step>, HashMap<&str, (&str, &str)>)> {

}

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

    #[rstest]
    #[case("RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)", 2)]
    #[case("LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)", 6)]
    #[trace]
    fn test_fn(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(process_part1(input).parse::<u32>().unwrap(), expected);
    }
}
