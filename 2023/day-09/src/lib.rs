use nom::{
    character::{
        self,
        complete::{newline, space1},
    },
    multi::separated_list1,
    IResult,
};

fn parse_sequence(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, character::complete::i64)(input)
}

fn parse_sequences(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(newline, parse_sequence)(input)
}

fn predict_value(seq: Vec<i64>, fwd: bool) -> i64 {
    let mut sequences = vec![seq];
    while !sequences.last().unwrap().iter().all(|v| v == &0) {
        sequences.push(
            sequences
                .last()
                .unwrap()
                .windows(2)
                .map(|vs| vs[1] - vs[0])
                .collect(),
        );
    }
    for idx in (0..sequences.len() - 2).rev() {
        if fwd {
            let next_value = *sequences[idx].last().unwrap() + *sequences[idx + 1].last().unwrap();
            sequences.get_mut(idx).unwrap().push(next_value)
        } else {
            let next_value = sequences[idx][0] - sequences[idx + 1][0];
            sequences.get_mut(idx).unwrap().insert(0, next_value)
        };
    }
    if fwd {
        *sequences[0].last().unwrap()
    } else {
        sequences[0][0]
    }
}

pub fn process_part1(input: &str) -> String {
    let (input, sequences) = parse_sequences(input).unwrap();
    debug_assert_eq!(input, "");
    sequences
        .into_iter()
        .map(|seq| predict_value(seq, true))
        .sum::<i64>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, sequences) = parse_sequences(input).unwrap();
    debug_assert_eq!(input, "");
    sequences
        .into_iter()
        .map(|seq| predict_value(seq, false))
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        "114"
    )]
    #[trace]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part1(input).as_str(), expected);
    }

    #[rstest]
    #[case(
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        "2"
    )]
    #[trace]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part2(input).as_str(), expected);
    }

    #[rstest]
    #[case("0 3 6 9 12 15", "18")]
    #[case("1 3 6 10 15 21", "28")]
    #[case("10 13 16 21 30 45", "68")]
    #[case("-1 -2 -3", "-4")]
    #[case("3 1 -1 -3", "-5")]
    #[trace]
    fn test_predict_next_value(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(
            predict_value(parse_sequence(input).unwrap().1, true)
                .to_string()
                .as_str(),
            expected
        );
    }

    #[rstest]
    #[case("0 3 6 9 12 15", "-3")]
    #[case("1 3 6 10 15 21", "0")]
    #[case("10 13 16 21 30 45", "5")]
    #[trace]
    fn test_predict_prev_value(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(
            predict_value(parse_sequence(input).unwrap().1, false)
                .to_string()
                .as_str(),
            expected
        );
    }
}
