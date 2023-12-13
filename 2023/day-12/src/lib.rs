use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline, space1},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};
use rayon::prelude::*;
use Record::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Record {
    Operational,
    Damaged,
    Unknown,
}

fn parse_row(input: &str) -> IResult<&str, (Vec<Record>, Vec<u32>)> {
    separated_pair(
        many1(alt((
            complete::char('#').map(|_| Damaged),
            complete::char('.').map(|_| Operational),
            complete::char('?').map(|_| Unknown),
        ))),
        space1,
        separated_list1(tag(","), complete::u32),
    )(input)
}

fn parse_rows(input: &str) -> IResult<&str, Vec<(Vec<Record>, Vec<u32>)>> {
    separated_list1(newline, parse_row)(input)
}

fn calculate_arrangements(
    records: Vec<Record>,
    groups: Vec<u32>,
) -> u32 {
    if records.is_empty() {
        return if groups.is_empty() { 1 } else { 0 };
    }

    let permutations = match *records.first().unwrap() {
        Operational => calculate_arrangements(records[1..].to_vec(), groups),
        Unknown => {
            let mut with_damaged = records.clone();
            with_damaged[0] = Damaged;
            let mut with_operational = records.clone();
            with_operational[0] = Operational;
            calculate_arrangements(with_damaged, groups.clone())
                + calculate_arrangements(with_operational, groups)
        }
        Damaged => {
            if groups.len() == 0 {
                0
            } else {
                let want_damaged = *groups.first().unwrap() as usize;

                if want_damaged <= records.len()
                    && records
                        .iter()
                        .take(want_damaged)
                        .all(|c| c == &Unknown || c == &Damaged)
                {
                    let new_groups = groups[1..].to_vec();
                    if want_damaged == records.len() {
                        if new_groups.is_empty() {
                            1
                        } else {
                            0
                        }
                    } else if records[want_damaged] == Operational {
                        calculate_arrangements(
                            records[want_damaged + 1..].to_vec(),
                            new_groups,
                       )
                    } else if records[want_damaged] == Unknown {
                        let mut new_records = records.clone();
                        new_records[want_damaged] = Operational;
                        calculate_arrangements(
                            new_records[want_damaged..].to_vec(),
                            new_groups,
                        )
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
        }
    };
    permutations
}

fn expand_row(records: Vec<Record>, groups: Vec<u32>) -> (Vec<Record>, Vec<u32>) {
    let new_records = std::iter::repeat(records.into_iter())
        .take(5)
        .intersperse(vec![Unknown].into_iter())
        .flatten()
        .collect_vec();
    let new_groups = std::iter::repeat(groups.into_iter())
        .take(5)
        .flatten()
        .collect_vec();
    (new_records, new_groups)
}

pub fn process_part1(input: &str) -> String {
    let (input, rows) = parse_rows(input).unwrap();
    debug_assert_eq!(input, "");
    rows.into_iter()
        .map(|(records, groups)| calculate_arrangements(records, groups))
        .sum::<u32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, rows) = parse_rows(input).unwrap();
    debug_assert_eq!(input, "");
    rows.into_par_iter()
        .map(|(records, groups)| expand_row(records, groups))
        .map(|(records, groups)| calculate_arrangements(records, groups))
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        "21"
    )]
    #[trace]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part1(input).as_str(), expected);
    }

    #[rstest]
    #[case("###.###.###.###.###.? 3,3,3,3,3,1", 1)]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    #[trace]
    fn test_arrangements(#[case] input: &str, #[case] expected: u32) {
        let (_, (records, groups)) = parse_row(input).unwrap();
        assert_eq!(calculate_arrangements(records, groups), expected);
    }

    #[rstest]
    #[case("???.### 1,1,3", "1")]
    #[case(".??..??...?##. 1,1,3", "16384")]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", "1")]
    #[case("????.#...#... 4,1,1", "16")]
    #[case("????.######..#####. 1,6,5", "2500")]
    #[case("?###???????? 3,2,1", "506250")]
    #[trace]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process_part2(input).as_str(), expected);
    }
}
